use aho_corasick::{AhoCorasick, MatchKind};
use linkify::{Link, LinkFinder, LinkKind};

//* Replacement Functions
pub fn replace_emails(text: &str) -> String {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Email]);

    let links = finder.links(&text).collect::<Vec<Link>>();
    let links = links
        .into_iter()
        .map(|link| link.as_str())
        .collect::<Vec<&str>>();
    let links_replace = vec![" (email) "; links.len()];

    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostLongest)
        .build(&links)
        .unwrap();
    ac.replace_all(&text, &links_replace)
}

pub fn replace_urls(text: &str) -> String {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    finder.url_must_have_scheme(false);

    let links = finder.links(&text).collect::<Vec<Link>>();
    let links = links
        .into_iter()
        .map(|link| link.as_str())
        .collect::<Vec<&str>>();
    let links_replace = vec![" (url) "; links.len()];

    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostLongest)
        .build(&links)
        .unwrap();
    ac.replace_all(&text, &links_replace)
}
