use std::{env, time::Instant};

use crate::emojis::*;
use other_patterns::*;
use polars::{
    lazy::dsl::{col, lit, GetOutput},
    prelude::*,
};
use spelling_corrector::*;
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
    // main
    env::set_var("POLARS_FMT_STR_LEN", "120");
    let dataset_filepath =
        "data/trainning_dataset/jigsaw-toxic-comment-classification-challenge/train.csv";

    let mut df = LazyCsvReader::new(dataset_filepath)
        .with_has_header(true)
        .finish()
        .unwrap()
        .with_column(col("comment_text").alias("m_ct"))
        // replace ip addresses
        .with_column(col("m_ct").str().replace_all(
            lit(r"((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.){3}(25[0-5]|(2[0-4]|1\d|[1-9]|)\d)"),
            lit(r" (ip address) "),
            false,
        ))
        // replace emails
        .with_column(col("m_ct").map(
            |s| {
                let ca = s.str()?;
                let out = ca.apply_to_buffer(replace_emails);
                Ok(Some(out.into_series()))
            },
            GetOutput::same_type(),
        ))
        // replace urls
        .with_column(col("m_ct").map(
            |s| {
                let ca = s.str()?;
                let out = ca.apply_to_buffer(replace_urls);
                Ok(Some(out.into_series()))
            },
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
            |s| {
                let ca = s.str()?;
                let out = ca.apply_to_buffer(replace_english_contractions);
                Ok(Some(out.into_series()))
            },
            GetOutput::same_type(),
        ))
        // replace wikipedia: shortcuts, (file) namespaces
        .with_column(col("m_ct").map(
            |s| {
                let ca = s.str()?;
                let out = ca.apply_to_buffer(replace_wikipedia_shortcuts);
                Ok(Some(out.into_series()))
            },
            GetOutput::same_type(),
        ))
        .with_column(col("m_ct").str().replace_all(
            lit(WIKIPEDIA_NAMESPACE_REGEX),
            lit(r" (wikipedia namespace) "),
            false,
        ))
        .with_column(col("m_ct").str().replace_all(
            lit(WIKIPEDIA_FILE_NAMESPACE_REGEX),
            lit(r" (wikipedia file namespace) "),
            false,
        ))
        // replace emoticons/emojis
        .with_column(col("m_ct").map(
            |s| {
                let ca = s.str()?;
                let out = ca.apply_to_buffer(replace_emoticons);
                Ok(Some(out.into_series()))
            },
            GetOutput::same_type(),
        ))
        .with_column(col("m_ct").map(
            |s| {
                let ca = s.str()?;
                let out = ca.apply_to_buffer(replace_unicode_emojis);
                Ok(Some(out.into_series()))
            },
            GetOutput::same_type(),
        ))
        // unicode filter: categories, blocks
        .with_column(col("m_ct").map(
            |s| {
                let ca = s.str()?;
                let out = ca.apply_to_buffer(unicode_filter_by_blocks);
                Ok(Some(out.into_series()))
            },
            GetOutput::same_type(),
        ))
        .with_column(col("m_ct").map(
            |s| {
                let ca = s.str()?;
                let out = ca.apply_to_buffer(unicode_filter_by_categories);
                Ok(Some(out.into_series()))
            },
            GetOutput::same_type(),
        ))
        // unicode: decode
        .with_column(col("m_ct").map(
            |s| {
                let ca = s.str()?;
                let out = ca.apply_to_buffer(unicode_decode);
                Ok(Some(out.into_series()))
            },
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
            |s| {
                let ca = s.str()?;
                let out = ca.apply_to_buffer(process_text);
                Ok(Some(out.into_series()))
            },
            GetOutput::same_type(),
        ))
        .collect()
        .unwrap();
    let mut file = std::fs::File::create("output.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();
    // let row = df.get_row(0).unwrap();
    // println!("{:?}", row.0[1]);
    // println!("{:?}", row.0[2]);
    //
    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
}
