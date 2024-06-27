use std::collections::HashMap;

use aho_corasick::{AhoCorasick, MatchKind};
use polars::{datatypes::StringChunked, error::PolarsError, prelude::*, series::Series};

// todo: rewrite using macro
pub fn series_str_map(
    series_str: Series,
    function: fn(&str) -> String,
) -> Result<Option<Series>, PolarsError> {
    let result = series_str
        .str()
        .unwrap()
        .into_iter()
        .map(|opt_str: Option<&str>| opt_str.map(|text: &str| function(text)))
        .collect::<StringChunked>();
    Ok(Some(result.into_series()))
}

// polars replace_many cannot configure MatchKind option, so I wrote my own
pub fn series_str_replace_all(
    series_str: Series,
    replace_hashmap: HashMap<String, String>,
) -> Result<Option<Series>, PolarsError> {
    let patterns = replace_hashmap.clone().into_keys().collect::<Vec<String>>();
    let replace_with = replace_hashmap
        .clone()
        .into_values()
        .collect::<Vec<String>>();
    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostLongest)
        .build(&patterns)
        .unwrap();
    let result = series_str
        .str()
        .unwrap()
        .into_iter()
        .map(|opt_str: Option<&str>| opt_str.map(|text: &str| ac.replace_all(&text, &replace_with)))
        .collect::<StringChunked>();
    Ok(Some(result.into_series()))
}
