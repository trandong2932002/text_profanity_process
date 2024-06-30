use deunicode::deunicode;
use std::fmt::Write;
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
pub fn get_unicode_block(letter: &char) -> &str {
    find_unicode_block(*letter).unwrap().name()
}

/// Normalize string.
pub fn unicode_normalize(text: &str, output: &mut String, normalization_form: Option<&str>) {
    let nf = normalization_form.unwrap_or("nfkd");
    let result = match nf {
        "nfc" => text.nfc().collect::<String>(),
        "nfd" => text.nfd().collect::<String>(),
        "nfkc" => text.nfkc().collect::<String>(),
        "nfkd" => text.nfkd().collect::<String>(),
        _ => "".to_owned(),
    };
    write!(output, "{}", result).unwrap();
}

/// Filter out all characters whose block is not accepted.
pub fn unicode_filter_by_blocks(text: &str, output: &mut String) {
    let result = text
        .chars()
        .filter(|letter| {
            ALLOWED_BLOCKS
                .iter()
                .any(|allowed_block| get_unicode_block(letter).contains(allowed_block))
        })
        .collect::<String>();
    write!(output, "{}", result).unwrap();
}

/// Filter out all characters whose category is not accepted.
pub fn unicode_filter_by_categories(text: &str, output: &mut String) {
    let result = text
        .chars()
        .filter(|letter| !NOT_ALLOWED_CATEGORIES.contains(&get_unicode_category(letter)))
        .collect::<String>();
    write!(output, "{}", result).unwrap();
}

/// Convert all characters to ASCII, also convert quotes/hyphens/dashes.
pub fn unicode_decode(text: &str, output: &mut String) {
    let result = deunicode(text);
    write!(output, "{}", result).unwrap();
}
