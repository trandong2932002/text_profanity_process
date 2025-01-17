use aho_corasick::{AhoCorasick, MatchKind};
use linkify::{LinkFinder, LinkKind};
use std::fmt::Write;

/// Filter out all emails from text, using Aho-Corasick algorithm.
pub fn replace_emails(text: &str, output: &mut String) {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Email]);

    let emails = finder
        .links(&text)
        .into_iter()
        .map(|link| link.as_str())
        .collect::<Vec<&str>>();
    let emails_replacement = vec![" (email) "; emails.len()];

    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostLongest)
        .build(&emails)
        .unwrap();
    let result = ac.replace_all(&text, &emails_replacement);
    write!(output, "{}", result).unwrap();
}

/// Filter out all urls from text, using Aho-Corasick algorithm.
pub fn replace_urls(text: &str, output: &mut String) {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    finder.url_must_have_scheme(false);

    let links = finder
        .links(&text)
        .into_iter()
        .map(|link| link.as_str())
        .collect::<Vec<&str>>();
    let links_replacement = vec![" (url) "; links.len()];

    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostLongest)
        .build(&links)
        .unwrap();
    let result = ac.replace_all(&text, &links_replacement);
    write!(output, "{}", result).unwrap();
}
