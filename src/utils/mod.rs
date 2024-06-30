use aho_corasick::{AhoCorasick, MatchKind};
use std::collections::HashMap;

pub fn aho_corasick_replace_all(
    text: &str,
    replacement_hashmap: &HashMap<String, String>,
) -> String {
    let patterns = replacement_hashmap
        .clone()
        .into_keys()
        .collect::<Vec<String>>();
    let replace_with = replacement_hashmap
        .clone()
        .into_values()
        .collect::<Vec<String>>();
    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostLongest)
        .build(patterns)
        .unwrap();
    ac.replace_all(&text, &replace_with)
}
