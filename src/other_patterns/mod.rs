use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt::Write, fs::File, io::BufReader};

use crate::aho_corasick_replace_all;

pub const WIKIPEDIA_NAMESPACE_REGEX: &str = r#"(talk|user|wikipedia|wp|project|wt|template|tm|help|category|portal|draft|timedtext|module|special|topic|education program|book|gadget|gadget definition)((_| )talk)?:[\w\/#]+"#;
pub const WIKIPEDIA_FILE_NAMESPACE_REGEX: &str =
    r#"(file|image)((_| )talk)?:([\w\s\(\)\&\-\"\']+)((\.(\w{3}))|,|\.|\)|\")"#;

static ENGLISH_CONTRACTIONS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let result = HashMap::from([
        ("won't", "will not"),
        ("can't", "can not"),
        ("n't", " not"),
        ("'re", " are"),
        ("'s", " is"),
        ("'d", " would"),
        ("'ll", " will"),
        ("'t", " not"),
        ("'ve", " have"),
        ("'m", " am"),
    ]);
    result
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect()
});

static WIKIPEDIA_SHORTCUTS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let wikipedia_shortcuts_filepath = "data/others/wiki_shortcuts.json";
    let file = File::open(wikipedia_shortcuts_filepath).unwrap();
    let reader = BufReader::new(file);
    let wikipedia_shortcuts: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    let keys = wikipedia_shortcuts
        .iter()
        .map(|(key, _)| (key.clone(), " (wikipedia shortcut) ".to_owned()))
        .collect::<HashMap<String, String>>();
    let values = wikipedia_shortcuts
        .iter()
        .map(|(_, value)| (value.clone(), " (wikipedia shortcut) ".to_owned()))
        .collect::<HashMap<String, String>>();
    let mut result = HashMap::new();
    result.extend(keys.to_owned());
    result.extend(values.to_owned());
    result
});

pub fn replace_english_contractions(text: &str, output: &mut String) {
    let result = aho_corasick_replace_all(text, &ENGLISH_CONTRACTIONS);
    write!(output, "{}", result).unwrap();
}

pub fn replace_wikipedia_shortcuts(text: &str, output: &mut String) {
    let result = aho_corasick_replace_all(text, &WIKIPEDIA_SHORTCUTS);
    write!(output, "{}", result).unwrap();
}
