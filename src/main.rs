use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    time::Instant,
};

use crate::emojis::*;
use other_patterns::*;
use polars::{
    lazy::dsl::{col, lit, GetOutput},
    prelude::*,
};
use spelling_corrector::*;
use symspell::{SymSpell, UnicodeStringStrategy};
use unicode::*;
use urls::{replace_emails, replace_urls};
use utils::*;

mod emojis;
mod other_patterns;
mod spelling_corrector;
mod unicode;
mod urls;
mod utils;

fn main() {
    let now = Instant::now();
    real_main();
    let elapsed = now.elapsed();
    println!("{:?}", elapsed);

    // test();
}

fn test() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_owned();
        println!("{} =======> {:?}", line, correct_unknown_word(&line));
    }
}

fn real_main() {
    env::set_var("POLARS_FMT_STR_LEN", "120");
    let dataset_filepath =
        "data/trainning_dataset/jigsaw-toxic-comment-classification-challenge/train.csv";

    let df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some(dataset_filepath.into()))
        .unwrap()
        .finish()
        .unwrap();
    let ct = df
        .lazy()
        .clone()
        // create new frame with columns [id, comment_text, m_ct = comment_text]
        .select([cols(["id", "comment_text"])])
        .with_column(col("comment_text").alias("m_ct"))
        // replace ip addresses
        .with_column(col("m_ct").str().replace_all(
            lit(r"((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.){3}(25[0-5]|(2[0-4]|1\d|[1-9]|)\d)"),
            lit(r" (ip address) "),
            false,
        ))
        // replace emails
        .with_column(col("m_ct").map(
            |series_str| series_str_map(series_str, replace_emails),
            GetOutput::same_type(),
        ))
        // replace urls
        .with_column(col("m_ct").map(
            |series_str| series_str_map(series_str, replace_urls),
            GetOutput::same_type(),
        ))
        // replace time (not date)
        .with_column(col("m_ct").str().replace_all(
            lit(r"(([0-9]|0[0-9]|1[0-9]|2[0-3]):[0-5][0-9](:[0-5][0-9])?)"),
            lit(r" (time) "),
            false,
        ))
        //? replace number with name?
        // replace english contractions
        .with_column(col("m_ct").map(
            move |series_str| {
                series_str_replace_all(series_str, get_english_contractions_hashmap())
            },
            GetOutput::same_type(),
        ))
        // replace wikipedia: shortcuts, (file) namespaces
        .with_column(col("m_ct").map(
            move |series_str| series_str_replace_all(series_str, get_wikipedia_shortcuts_hashmap()),
            GetOutput::same_type(),
        ))
        .with_column(col("m_ct").str().replace_all(
            lit(get_wikipedia_namespace_regex()),
            lit(r" (wikipedia namespace) "),
            false,
        ))
        .with_column(col("m_ct").str().replace_all(
            lit(get_wikipedia_file_namespace_regex()),
            lit(r" (wikipedia file namespace) "),
            false,
        ))
        // replace emoticons/emojis
        .with_column(col("m_ct").map(
            move |series_str| series_str_replace_all(series_str, get_emoticons_hashmap()),
            GetOutput::same_type(),
        ))
        .with_column(col("m_ct").map(
            move |series_str| series_str_replace_all(series_str, get_unicode_emojis_hashmap()),
            GetOutput::same_type(),
        ))
        // unicode filter: categories, blocks
        .with_column(col("m_ct").map(
            |series_str| series_str_map(series_str, unicode_filter_by_categories),
            GetOutput::same_type(),
        ))
        .with_column(col("m_ct").map(
            |series_str| series_str_map(series_str, unicode_filter_by_blocks),
            GetOutput::same_type(),
        ))
        // unicode: decode
        .with_column(col("m_ct").map(
            |series_str| series_str_map(series_str, unicode_decode),
            GetOutput::same_type(),
        ))
        // split punctuations/symbols around word
        .with_column(col("m_ct").str().replace_all(
            lit(r"\b([\w\d]?[^\s]*[\w\d]?)\b"),
            lit(r" ${1} "),
            false,
        ))
        // other process
        .with_column(col("m_ct").map(
            |series_str| series_str_map(series_str, process_text),
            GetOutput::same_type(),
        ));

    //--
    let mut df = ct.collect().unwrap();
    let mut file = std::fs::File::create("data/trainning_dataset/output/output.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();
    // let row = df.get_row(0).unwrap();
    // println!("{:?}", row.0[1]);
    // println!("{:?}", row.0[2]);
}
