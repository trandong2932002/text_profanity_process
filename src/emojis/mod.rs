use emojis::Emoji;
use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt::Write, fs::File, io::BufReader};

use crate::aho_corasick_replace_all;

const UNICODE_VERSION_MAJOR: u32 = 15;
const UNICODE_VERSION_MINOR: u32 = 1;

pub static EMOTICONS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let emoticons_filepath = "data/emojis/combined_emoji_vi.json";
    let file = File::open(emoticons_filepath).unwrap();
    let reader = BufReader::new(file);
    let emoticons_json: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    emoticons_json
        .iter()
        .filter_map(|(emoticon, emoticon_name)| {
            if emoticon.chars().all(|c| c.is_ascii_alphabetic()) {
                return None;
            }
            Some((emoticon.to_owned(), format!(" {} ", emoticon_name)))
        })
        .collect()
});

pub static UNICODE_EMOJIS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    // emojis::iter()
    //     .filter_map(|e| {
    //         if e.unicode_version()
    //             > emojis::UnicodeVersion::new(UNICODE_VERSION_MAJOR, UNICODE_VERSION_MINOR)
    //         {
    //             return None;
    //         }
    //         let e_skin_tones = e.skin_tones();
    //         if e_skin_tones.is_none() {
    //             return None;
    //         }
    //         Some(e_skin_tones.unwrap().collect::<Vec<&Emoji>>())
    //     })
    //     .flatten()
    //     .map(|e| {
    //         (
    //             e.as_str().to_owned(),
    //             format!(" ({}) ", e.name().to_owned()),
    //         )
    //     })
    //     .collect()
    let emoticons_filepath = "data/emojis/unicode_emoji_vi.json";
    let file = File::open(emoticons_filepath).unwrap();
    let reader = BufReader::new(file);
    let emoticons_json: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    emoticons_json
});

pub fn replace_emoticons(text: &str, output: &mut String) {
    let result = aho_corasick_replace_all(text, &EMOTICONS);
    write!(output, "{}", result).unwrap();
}

pub fn replace_unicode_emojis(text: &str, output: &mut String) {
    let result = aho_corasick_replace_all(text, &UNICODE_EMOJIS);
    write!(output, "{}", result).unwrap();
}
