use deunicode::{deunicode, deunicode_char};
use std::{fmt::Write, result};
use unicode_blocks::find_unicode_block;
use unicode_normalization::UnicodeNormalization;
use unicode_properties::{GeneralCategoryGroup, UnicodeGeneralCategory};

const NOT_ALLOWED_CATEGORIES: [char; 1] = ['C'];
// shortname of allowed blocks
const ALLOWED_BLOCKS: [&str; 7] = [
    "Latin",
    "Greek",
    "Phonetic",
    "Spacing",
    "General Punctuation",
    "Currency Symbols",
    "IPA",
];
const VIETNAMESE_ALLOWED_CHARS: [char; 178] = [
    'A', 'a', 'Á', 'á', 'À', 'à', 'Ả', 'ả', 'Ã', 'ã', 'Ạ', 'ạ', 'Ă', 'ă', 'Ắ', 'ắ', 'Ằ', 'ằ', 'Ẳ',
    'ẳ', 'Ẵ', 'ẵ', 'Ặ', 'ặ', 'Â', 'â', 'Ấ', 'ấ', 'Ầ', 'ầ', 'Ẩ', 'ẩ', 'Ẫ', 'ẫ', 'Ậ', 'ậ', 'B', 'b',
    'C', 'c', 'D', 'd', 'Đ', 'đ', 'E', 'e', 'É', 'é', 'È', 'è', 'Ẻ', 'ẻ', 'Ẽ', 'ẽ', 'Ẹ', 'ẹ', 'Ê',
    'ê', 'Ế', 'ế', 'Ề', 'ề', 'Ể', 'ể', 'Ễ', 'ễ', 'Ệ', 'ệ', 'G', 'g', 'H', 'h', 'I', 'i', 'Í', 'í',
    'Ì', 'ì', 'Ỉ', 'ỉ', 'Ĩ', 'ĩ', 'Ị', 'ị', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Ó',
    'ó', 'Ò', 'ò', 'Ỏ', 'ỏ', 'Õ', 'õ', 'Ọ', 'ọ', 'Ô', 'ô', 'Ố', 'ố', 'Ồ', 'ồ', 'Ổ', 'ổ', 'Ỗ', 'ỗ',
    'Ộ', 'ộ', 'Ơ', 'ơ', 'Ớ', 'ớ', 'Ờ', 'ờ', 'Ở', 'ở', 'Ỡ', 'ỡ', 'Ợ', 'ợ', 'P', 'p', 'Q', 'q', 'R',
    'r', 'S', 's', 'T', 't', 'U', 'u', 'Ú', 'ú', 'Ù', 'ù', 'Ủ', 'ủ', 'Ũ', 'ũ', 'Ụ', 'ụ', 'Ư', 'ư',
    'Ứ', 'ứ', 'Ừ', 'ừ', 'Ử', 'ử', 'Ữ', 'ữ', 'Ự', 'ự', 'V', 'v', 'X', 'x', 'Y', 'y', 'Ý', 'ý', 'Ỳ',
    'ỳ', 'Ỷ', 'ỷ', 'Ỹ', 'ỹ', 'Ỵ', 'ỵ',
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

/// Normalize string (nfkc).
pub fn unicode_normalize(text: &str, output: &mut String) {
    let result = text.nfkc().collect::<String>();
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

pub fn unicode_decode_vietnamese(text: &str, output: &mut String) {
    let mut new_chars = Vec::new();
    for char in text.chars() {
        if VIETNAMESE_ALLOWED_CHARS.contains(&char) {
            new_chars.push(char.to_string());
        } else {
            new_chars.push(deunicode(&char.to_string()));
        }
    }
    let result = new_chars.join("");
    write!(output, "{}", result).unwrap();
}
