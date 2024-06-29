use std::{
    borrow::Borrow,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use aho_corasick::{AhoCorasick, MatchKind};
use once_cell::sync::Lazy;
use rust_stemmers::{Algorithm, Stemmer};
use symspell::{SymSpell, UnicodeStringStrategy, Verbosity};

use crate::get_unicode_category;

const BIGRAM_DUPLICATE_THRESHOLD: f32 = 0.3; // magic number

// const ENGLISH_ONE_LETTER_WORDS: [char; 12] =
//     ['a', 'i', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
// const ENGLISH_TWO_LETTERS_WORDS: [&str; 26] = [
//     "am", "an", "as", "at", "be", "by", "do", "go", "he", "if", "in", "is", "it", "me", "my", "no",
//     "of", "ok", "on", "or", "ox", "so", "to", "up", "us", "we",
// ];
// const ENGLISH_AVG_WORD_LENGTH: i32 = 5;
// const ENGLISH_WORD_LENGTH_THRESHOLD: i32 = 2 * ENGLISH_AVG_WORD_LENGTH;
const ENGLISH_SEGMENTATION_MAX_EDIT_DISTANCE: i64 = 2;

pub static SYMSPELL: Lazy<SymSpell<UnicodeStringStrategy>> = Lazy::new(|| {
    eprintln!("Spelling Corrector: SymSpell loading...");
    let mut spell = SymSpell::default();
    let english_frequency_filepath = "data/dictionaries/english/frequency_dictionary_en_82_765.txt";
    let english_bigram_frequency_filepath =
        "data/dictionaries/english/frequency_bigramdictionary_en_243_342.txt";
    spell.load_dictionary(&english_frequency_filepath, 0, 1, " ");
    spell.load_bigram_dictionary(&english_bigram_frequency_filepath, 0, 2, " ");
    spell
});

static ENGLISH_DICTIONARY: Lazy<HashSet<String>> = Lazy::new(|| {
    eprintln!("Spelling Corrector: English dictionary loading...");
    let mut dictionary: HashSet<String> = HashSet::new();
    let english_dictionary_filepath = "data/dictionaries/english/words_alpha.txt";
    let file = File::open(english_dictionary_filepath).unwrap();
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

static ENGLISH_FIRSTNAMES: Lazy<Vec<String>> = Lazy::new(|| {
    eprintln!("Spelling Corrector: English firtnames loading...");
    let mut firstnames: Vec<String> = Vec::new();
    let english_firstnames_filepath = "data/dictionaries/english/first-names.txt";
    let file = File::open(english_firstnames_filepath).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap().to_lowercase();
        firstnames.push(line);
    }
    firstnames
});

static ENGLISH_FIRSTNAMES_REPLACEMENT: Lazy<Vec<String>> = Lazy::new(|| {
    eprintln!("Spelling Corrector: English swear words replacement creating...");
    ENGLISH_FIRSTNAMES
        .clone()
        .into_iter()
        // .filter(|word| !word.contains(" "))
        .map(|word| format!(" {} ", word))
        .collect()
});

static ENGLISH_STEMMER: Lazy<Stemmer> = Lazy::new(|| {
    eprintln!("Spelling Corrector: English stemmer loading...");
    Stemmer::create(Algorithm::English)
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
    ENGLISH_DICTIONARY.contains(word)
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

// Check if a word contains only punctuations and symbols.
fn is_punctuations_or_symbols(word: &str) -> bool {
    word.chars()
        .into_iter()
        .all(|letter| ['P', 'S'].contains(&get_unicode_category(&letter)))
}

/// Algorithm to correct unknown word
pub fn correct_unknown_word(word: &str) -> String {
    // first: with simple unknown word, replace all punctuations/symbols with space, try to correct
    // ex: hello.how.are.you
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
    if new_word != word {
        let mut result_words: Vec<String> = Vec::new();
        let mut can_correct_flag = true;
        for _word in new_word.split_whitespace().into_iter() {
            if is_in_corpora(_word) {
                result_words.push(_word.to_owned());
                continue;
            }
            let suggestion = SYMSPELL.lookup(_word, Verbosity::Top, 2);
            if suggestion.len() == 0 {
                can_correct_flag = false;
                break;
            }
            result_words.push(suggestion[0].term.to_owned());
        }
        if can_correct_flag {
            // print!("1>");
            return result_words.join(" ");
        }
    }
    // second: this word may be complex (multiple words and wrong spell), split and try to correct
    // when split misspelling word, the result may contains unknown sequences of 1/2-chars words (not yet impl)
    // ex: he.l.loh.o.w.ar.ey.ou
    // print!("3>");
    new_word.retain(|letter| !letter.is_whitespace());
    new_word = reduce_bigram(&new_word);
    // replace swear words
    //todo: whitelist?
    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostLongest)
        .build(&ENGLISH_SWEAR_WORDS.to_owned())
        .unwrap();
    new_word = ac.replace_all(&new_word, &ENGLISH_SWEAR_WORDS_REPLACEMENT);
    // replace firstnames
    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostLongest)
        .build(&ENGLISH_FIRSTNAMES.to_owned())
        .unwrap();
    new_word = ac.replace_all(&new_word, &ENGLISH_FIRSTNAMES_REPLACEMENT);
    // split text
    let segmented_string = SYMSPELL
        .word_segmentation(&new_word, ENGLISH_SEGMENTATION_MAX_EDIT_DISTANCE)
        .segmented_string;
    segmented_string
}

pub fn process_text(text: &str) -> String {
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
        if is_in_corpora(ENGLISH_STEMMER.stem(&word).borrow()) {
            result_words.push(word);
            continue;
        }
        result_words.push(correct_unknown_word(&word));
    }
    result_words.join(" ")
}
