use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
};

use deunicode::deunicode_with_tofu;
use emojis::*;
use spelling_corrector::*;
use unicode::*;
use urls::*;
use utils::*;

mod emojis;
mod other_patterns;
mod spelling_corrector;
mod unicode;
mod urls;
mod utils;

fn main() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        // line = line.trim().to_lowercase();
        let result = text_profanity_process(&line);
        println!("{}", result);
    }
}

fn text_profanity_process(text: &str) -> String {
    let mut new_text = text.to_owned();
    new_text = apply(&new_text, unicode_normalize);
    new_text = apply(&new_text, replace_emails);
    new_text = apply(&new_text, replace_urls);
    new_text = apply(&new_text, replace_emoticons);
    new_text = apply(&new_text, replace_unicode_emojis);
    new_text = apply(&new_text, unicode_filter_by_blocks);
    new_text = apply(&new_text, unicode_filter_by_categories);
    new_text = apply(&new_text, unicode_decode_vietnamese);
    new_text = apply(&new_text, process_text);
    new_text
}
