use std::io;

use crate::emojis::*;
use spelling_corrector::*;
use unicode::*;
use unicode_normalization::UnicodeNormalization;
use urls::{replace_emails, replace_urls};
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
        line = line.trim().to_lowercase();
        line = line.nfkc().collect::<String>();
        line = apply(&line, replace_emails);
        line = apply(&line, replace_urls);
        line = apply(&line, replace_emoticons);
        line = apply(&line, replace_unicode_emojis);
        line = apply(&line, unicode_filter_by_blocks);
        line = apply(&line, unicode_filter_by_categories);
        line = apply(&line, process_text);

        println!("{}", line);
    }
}

fn apply<'a, F>(line: &'a str, mut f: F) -> String
where
    F: FnMut(&'a str, &mut String),
{
    let mut buf = String::new();
    f(line, &mut buf);
    buf
}
