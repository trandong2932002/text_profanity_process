use deunicode::deunicode;
use unicode_blocks::find_unicode_block;
use unicode_normalization::UnicodeNormalization;
use unicode_properties::{GeneralCategoryGroup, UnicodeGeneralCategory};

const NOT_ALLOWED_CATEGORIES: [char; 1] = ['C'];
// shortname of allowed blocks
const ALLOWED_BLOCKS: [&str; 6] = [
    "Latin",
    "Greek",
    "Phonetic",
    "Spacing",
    "General Punctuation",
    "Currency Symbols",
];

/// Get unicode category of character.
pub fn get_unicode_category(letter: &char) -> char {
    match letter.general_category_group() {
        GeneralCategoryGroup::Letter => 'L',
        GeneralCategoryGroup::Mark => 'M',
        GeneralCategoryGroup::Number => 'N',
        GeneralCategoryGroup::Punctuation => 'P',
        GeneralCategoryGroup::Symbol => 'S',
        GeneralCategoryGroup::Separator => 'Z',
        GeneralCategoryGroup::Other => 'C',
    }
}

/// Get unicode block of character.
pub fn get_unicode_block(letter: &char) -> String {
    find_unicode_block(*letter).unwrap().name().to_owned()
}

/// Normalize string.
pub fn unicode_normalize(text: &str, normalization_form: Option<&str>) -> String {
    let nf = normalization_form.unwrap_or("nfkd");
    match nf {
        "nfc" => text.nfc().collect(),
        "nfd" => text.nfd().collect(),
        "nfkc" => text.nfkc().collect(),
        "nfkd" => text.nfkd().collect(),
        _ => {
            println!("what is going on there?");
            text.nfkd().collect()
        }
    }
}

/// Filter out all characters whose block is not accepted.
pub fn unicode_filter_by_blocks(text: &str) -> String {
    text.chars()
        .filter(|letter| {
            ALLOWED_BLOCKS
                .iter()
                .any(|allowed_block| get_unicode_block(letter).contains(allowed_block))
        })
        .collect()
}

/// Filter out all characters whose category is not accepted.
pub fn unicode_filter_by_categories(text: &str) -> String {
    text.chars()
        .filter(|letter| !NOT_ALLOWED_CATEGORIES.contains(&get_unicode_category(letter)))
        .collect()
}

/// Convert all characters to ASCII, also convert quotes/hyphens/dashes.
pub fn unicode_decode(text: &str) -> String {
    deunicode(text)
}
