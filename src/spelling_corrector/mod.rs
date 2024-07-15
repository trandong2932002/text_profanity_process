use std::{
    collections::HashSet,
    fmt::Write,
    fs::File,
    io::{BufRead, BufReader},
};

use aho_corasick::{AhoCorasick, MatchKind};
use once_cell::sync::Lazy;
use symspell::{SymSpell, UnicodeStringStrategy, Verbosity};
use unicode_segmentation::UnicodeSegmentation;

use crate::{apply, get_unicode_category, unicode_decode};

const BIGRAM_DUPLICATE_THRESHOLD: f32 = 0.3; // magic number
const SEGMENTATION_MAX_EDIT_DISTANCE: i64 = 2;

// todo: generate new data using https://github.com/binhvq/news-corpus
pub static SYMSPELL: Lazy<SymSpell<UnicodeStringStrategy>> = Lazy::new(|| {
    eprintln!("Spelling Corrector: SymSpell loading...");
    let mut spell = SymSpell::default();
    let vietnamese_frequency_filepath = "data/dictionaries/vietnamese/vi_50k.txt";
    spell.load_dictionary(&vietnamese_frequency_filepath, 0, 1, " ");
    spell
});

pub static SYMSPELL_WITHOUT_ACCENTS: Lazy<SymSpell<UnicodeStringStrategy>> = Lazy::new(|| {
    eprintln!("Spelling Corrector: SymSpell (without accents) loading...");
    let mut spell = SymSpell::default();
    let vietnamese_frequency_filepath = "data/dictionaries/vietnamese/vi_50k_no_accent.txt";
    spell.load_dictionary(&vietnamese_frequency_filepath, 0, 1, " ");
    spell
});

static VIETNAMESE_DICTIONARY: Lazy<HashSet<String>> = Lazy::new(|| {
    eprintln!("Spelling Corrector: Vietnamese dictionary loading...");
    let mut dictionary: HashSet<String> = HashSet::new();
    let vietnamese_dictionary_filepath = "data/dictionaries/vietnamese/words_alpha.txt";
    let file = File::open(vietnamese_dictionary_filepath).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        dictionary.insert(line.unwrap());
    }
    dictionary
});

static ENGLISH_SWEAR_WORDS: Lazy<Vec<String>> = Lazy::new(|| {
    eprintln!("Spelling Corrector: English swear words loading...");
    let mut swear_words: Vec<String> = Vec::new();
    let english_swear_words_filepath = "data/dictionaries/english/profanity_wordlist.txt";
    let file = File::open(english_swear_words_filepath).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap().to_lowercase();
        swear_words.push(line);
    }
    swear_words
});

static ENGLISH_SWEAR_WORDS_REPLACEMENT: Lazy<Vec<String>> = Lazy::new(|| {
    eprintln!("Spelling Corrector: English swear words replacement creating...");
    ENGLISH_SWEAR_WORDS
        .clone()
        .into_iter()
        // .filter(|word| !word.contains(" "))
        .map(|word| format!(" {} ", word))
        .collect()
});

static VIETNAMESE_SWEAR_WORDS: Lazy<Vec<String>> = Lazy::new(|| {
    eprintln!("Spelling Corrector: Vietnamese swear words loading...");
    let mut swear_words: HashSet<String> = HashSet::new();
    let vietnamese_swear_words_filepath = "data/dictionaries/vietnamese/vi.json";
    let file = File::open(vietnamese_swear_words_filepath).unwrap();
    let reader = BufReader::new(file);
    let s_words: Vec<String> = serde_json::from_reader(reader).unwrap();
    swear_words.extend(s_words);

    let vietnamese_swear_words_filepath = "data/dictionaries/vietnamese/vn_offensive_words.txt";
    let file = File::open(vietnamese_swear_words_filepath).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let word = line.unwrap();
        if word.chars().nth(0).unwrap() != '#' {
            swear_words.insert(word);
        }
    }
    swear_words.into_iter().collect::<Vec<String>>()
});

static VIETNAMESE_SWEAR_WORDS_REPLACEMENT: Lazy<Vec<String>> = Lazy::new(|| {
    eprintln!("Spelling Corrector: Vietnamese swear words replacement creating...");
    VIETNAMESE_SWEAR_WORDS
        .clone()
        .into_iter()
        // .filter(|word| !word.contains(" "))
        .map(|word| format!(" {} ", word))
        .collect()
});

/// Create vector of bigrams, e.g. hello => he,el,ll,lo
fn split_bigram(word: &str) -> Vec<[char; 2]> {
    //* Vietnamese and English do not need graphemes
    // // let letters = word.graphemes(true).collect::<Vec<&str>>();
    // // let letters0 = letters.split_last().unwrap().1;
    // // let letters1 = letters.split_first().unwrap().1;
    let letters = word.chars().into_iter().collect::<Vec<char>>();
    let letters0 = letters.split_last().unwrap().1;
    let letters1 = letters.split_first().unwrap().1;
    let mut bigrams = Vec::new();
    for (letter0, letter1) in letters0.iter().zip(letters1) {
        let bigram = [*letter0, *letter1];
        bigrams.push(bigram);
    }
    bigrams
}

/// Create string from bigrams, e.g. he,el,ll,lo => hello
fn join_bigram(bigrams: &Vec<[char; 2]>) -> String {
    let mut new_word = String::new();
    for bigram in bigrams.iter() {
        new_word.push(bigram[0]);
    }
    new_word.push(bigrams.last().unwrap()[1]);
    new_word
}

/// Remove duplicate bigrams, e.g. helllo => he,el,ll,ll,lo => he,el,ll,lo => hello
/// If there are too many duplicate bigrams in a word, remove all duplicate bigrams, e.g. hheelloo => helo
fn reduce_bigram(word: &str) -> String {
    let bigrams = split_bigram(word);
    let mut new_bigrams: Vec<[char; 2]> = Vec::new();
    // keep first group of duplicate bigram
    let mut repeat_flag = false;
    let mut duplication_num = 0;
    for bigram in bigrams.iter() {
        if bigram[0] == bigram[1] {
            if repeat_flag {
                continue;
            }
            repeat_flag = true;
            duplication_num += 1;
        } else {
            repeat_flag = false;
        }
        new_bigrams.push(*bigram);
    }
    // check if it have too many duplicate bigrams
    let len_bigrams = new_bigrams.len() + 1;
    if len_bigrams > 3 && duplication_num as f32 > len_bigrams as f32 * BIGRAM_DUPLICATE_THRESHOLD {
        let mut new_new_bigrams: Vec<[char; 2]> = Vec::new();
        for bigram in new_bigrams.iter() {
            if bigram[0] == bigram[1] {
                continue;
            }
            new_new_bigrams.push(*bigram);
        }
        return join_bigram(&new_new_bigrams);
    }
    join_bigram(&new_bigrams)
}

/// Check if a word is in the corpora.
fn is_in_corpora(word: &str) -> bool {
    VIETNAMESE_DICTIONARY.contains(word)
}

/// Check if a word is a number.
fn is_a_number(word: &str) -> bool {
    match word.parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Check if a word is something like a math equation (combination of numbers, symbols and punctuations).
fn is_math_equation(word: &str) -> bool {
    word.chars()
        .into_iter()
        .all(|letter| ['N', 'S', 'P'].contains(&get_unicode_category(&letter)))
}

/// Check if a word contains only punctuations and symbols.
fn is_punctuations_or_symbols(word: &str) -> bool {
    word.chars()
        .into_iter()
        .all(|letter| ['P', 'S'].contains(&get_unicode_category(&letter)))
}

/// Sympell word segmentation for vietnamese is better without accents
fn word_segmentation_without_accents(text: &str, spell: &str) -> String {
    let c_text = text.graphemes(true).collect::<Vec<&str>>();
    let indices = spell
        .match_indices(' ')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let mut result: Vec<String> = Vec::new();
    let mut last_index = 0;
    for (i, index) in indices.iter().enumerate() {
        result.push(c_text[last_index..(index - i)].join(""));
        last_index = index - i;
    }
    result.push(c_text[last_index..].join(""));
    result.join(" ")
}

/// Algorithm to correct unknown word
pub fn correct_unknown_word(word: &str) -> String {
    let mut new_word = word
        .chars()
        .map(|letter| {
            if ['L', 'N'].contains(&get_unicode_category(&letter)) {
                letter
            } else {
                ' '
            }
        })
        .collect::<String>();
    //? single word?
    new_word.retain(|letter| !letter.is_whitespace()); //?
    new_word = reduce_bigram(&new_word);

    // replace swear words
    // todo: whitelist?
    // todo: correct replacement?
    // let ac = AhoCorasick::builder()
    //     .ascii_case_insensitive(true)
    //     .match_kind(MatchKind::LeftmostLongest)
    //     // .build(&ENGLISH_SWEAR_WORDS.to_owned())
    //     .build(&ENGLISH_SWEAR_WORDS.to_owned())
    //     .unwrap();
    // new_word = ac.replace_all(&new_word, &ENGLISH_SWEAR_WORDS_REPLACEMENT);

    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostLongest)
        // .build(&ENGLISH_SWEAR_WORDS.to_owned())
        .build(&VIETNAMESE_SWEAR_WORDS.to_owned())
        .unwrap();
    new_word = ac.replace_all(&new_word, &VIETNAMESE_SWEAR_WORDS_REPLACEMENT);

    // multiple words stuck together: split text
    let spell0 = SYMSPELL_WITHOUT_ACCENTS.word_segmentation(
        &apply(&new_word, unicode_decode),
        SEGMENTATION_MAX_EDIT_DISTANCE,
    );
    let spell1 = SYMSPELL.word_segmentation(&new_word, SEGMENTATION_MAX_EDIT_DISTANCE);
    if spell0.segmented_string == apply(&spell1.segmented_string, unicode_decode) {
        return spell1.segmented_string;
    }
    //
    if spell0.distance_sum < spell1.distance_sum {
        // select spell0
        return word_segmentation_without_accents(&new_word, &spell0.segmented_string);
    } else if spell0.distance_sum > spell1.distance_sum {
        // select spell1
        return spell1.segmented_string;
    } else if spell0.prob_log_sum < spell1.prob_log_sum {
        // select spell0
        return word_segmentation_without_accents(&new_word, &spell0.segmented_string);
    } else if spell0.prob_log_sum > spell0.prob_log_sum {
        // select spell1
        return spell1.segmented_string;
    } else {
        // default: select spell1
        return spell1.segmented_string;
    }
}

pub fn process_text(text: &str, output: &mut String) {
    let mut result_words: Vec<String> = Vec::new();
    for word in text.split_whitespace().into_iter() {
        let word = word.to_lowercase();
        if is_a_number(&word) {
            result_words.push(word);
            continue;
        }
        if is_punctuations_or_symbols(&word) {
            result_words.push(word);
            continue;
        }
        if is_math_equation(&word) {
            result_words.push(word);
            continue;
        }
        if is_in_corpora(&word) {
            result_words.push(word);
            continue;
        }
        if is_in_corpora(&word) {
            result_words.push(word);
            continue;
        }
        result_words.push(correct_unknown_word(&word));
    }
    let result = result_words.join(" ");
    write!(output, "{}", result).unwrap();
}
