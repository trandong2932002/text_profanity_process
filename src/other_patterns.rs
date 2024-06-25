use std::{collections::HashMap, fs::File, io::BufReader};

//* Patterns
pub fn get_english_contractions_hashmap() -> HashMap<String, String> {
    let mut english_contractions = HashMap::new();
    english_contractions.insert("won't".to_owned(), "will not".to_owned());
    english_contractions.insert("can't".to_owned(), "can not".to_owned());
    english_contractions.insert("n't".to_owned(), " not".to_owned());
    english_contractions.insert("'re".to_owned(), " are".to_owned());
    english_contractions.insert("'s".to_owned(), " is".to_owned());
    english_contractions.insert("'d".to_owned(), " would".to_owned());
    english_contractions.insert("'ll".to_owned(), " will".to_owned());
    english_contractions.insert("'t".to_owned(), " not".to_owned());
    english_contractions.insert("'ve".to_owned(), " have".to_owned());
    english_contractions.insert("'m".to_owned(), " am".to_owned());
    english_contractions
}

pub fn get_wikipedia_shortcuts_hashmap() -> HashMap<String, String> {
    let wikipedia_shortcuts_filepath = "data/wikipedia_shortcuts/wiki_shortcuts.json";
    let file = File::open(wikipedia_shortcuts_filepath).unwrap();
    let reader = BufReader::new(file);
    let wikipedia_shortcuts: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    let keys = get_wikipedia_shortcuts_keys_hashmap(&wikipedia_shortcuts);
    let values = get_wikipedia_shortcuts_values_hashmap(&wikipedia_shortcuts);
    let mut result = keys;
    result.extend(values);
    result
}

fn get_wikipedia_shortcuts_keys_hashmap(
    wikipedia_shortcuts: &HashMap<String, String>,
) -> HashMap<String, String> {
    wikipedia_shortcuts
        .clone()
        .into_keys()
        .map(|keys| (keys, " (wikipedia shortcut) ".to_owned()))
        .collect()
}

fn get_wikipedia_shortcuts_values_hashmap(
    wikipedia_shortcuts: &HashMap<String, String>,
) -> HashMap<String, String> {
    wikipedia_shortcuts
        .clone()
        .into_values()
        .map(|values| (values, " (wikipedia shortcut) ".to_owned()))
        .collect()
}

pub fn get_wikipedia_namespace_regex() -> String {
    r#"(talk|user|wikipedia|wp|project|wt|template|tm|help|category|portal|draft|timedtext|module|special|topic|education program|book|gadget|gadget definition)((_| )talk)?:[\w\/#]+"#.to_owned()
}

pub fn get_wikipedia_file_namespace_regex() -> String {
    r#"(file|image)((_| )talk)?:([\w\s\(\)\&\-\"\']+)((\.(\w{3}))|,|\.|\)|\")"#.to_owned()
}
