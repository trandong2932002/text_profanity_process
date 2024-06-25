use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use once_cell::sync::Lazy;
use symspell::{SymSpell, UnicodeStringStrategy, Verbosity};

use crate::get_unicode_category;

const BIGRAM_DUPLICATE_THRESHOLD: f32 = 0.3; // magic number

const ENGLISH_FREQUENCY_DICTIONARY_FILEPATH: &str =
    "data/english/frequency_dictionary_en_82_765.txt";
const ENGLISH_FREQUENCY_BIGRAM_DICTIONARY_FILE_PATH: &str =
    "data/english/frequency_bigramdictionary_en_243_342.txt";
const ENGLISH_DICTIONARY_FILEPATH: &str = "data/english/words_alpha.txt";
// const ENGLISH_ONE_LETTER_WORDS: [char; 12] =
//     ['a', 'i', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
// const ENGLISH_TWO_LETTERS_WORDS: [&str; 26] = [
//     "am", "an", "as", "at", "be", "by", "do", "go", "he", "if", "in", "is", "it", "me", "my", "no",
//     "of", "ok", "on", "or", "ox", "so", "to", "up", "us", "we",
// ];
// const ENGLISH_AVG_WORD_LENGTH: i32 = 5;
// const ENGLISH_WORD_LENGTH_THRESHOLD: i32 = 2 * ENGLISH_AVG_WORD_LENGTH;
const ENGLISH_SEGMENTATION_MAX_EDIT_DISTANCE: i64 = 2;

static symspell: Lazy<SymSpell<UnicodeStringStrategy>> = Lazy::new(|| {
    eprintln!("Symspell loading...");
    let mut spell = SymSpell::default();
    spell.load_dictionary(ENGLISH_FREQUENCY_DICTIONARY_FILEPATH, 0, 1, " ");
    spell.load_bigram_dictionary(ENGLISH_FREQUENCY_BIGRAM_DICTIONARY_FILE_PATH, 0, 2, " ");
    spell
});

static english_dictionary: Lazy<HashSet<String>> = Lazy::new(|| {
    eprintln!("English dictionary loading...");
    let mut dictionary: HashSet<String> = HashSet::new();
    let file = File::open(ENGLISH_DICTIONARY_FILEPATH).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        dictionary.insert(line.unwrap());
    }
    dictionary
});

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

fn join_bigram(bigrams: &Vec<[char; 2]>) -> String {
    let mut new_word = String::new();
    for bigram in bigrams.iter() {
        new_word.push(bigram[0]);
    }
    new_word.push(bigrams.last().unwrap()[1]);
    new_word
}

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
    // check if it have too much bigram duplications
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

fn is_in_corpora(word: &str) -> bool {
    english_dictionary.contains(word)
}

fn is_a_number(word: &str) -> bool {
    match word.parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_math_equation(word: &str) -> bool {
    word.chars()
        .into_iter()
        .all(|letter| ['N', 'S', 'P'].contains(&get_unicode_category(&letter)))
}

fn is_punctuations_or_symbols(word: &str) -> bool {
    word.chars()
        .into_iter()
        .all(|letter| ['P', 'S'].contains(&get_unicode_category(&letter)))
}

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
            let suggestion = symspell.lookup(_word, Verbosity::Top, 2);
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
    // // second: if this word is single word (word length small), try to correct
    // // ex: f.u.c.k, f*ck
    new_word.retain(|letter| !letter.is_whitespace());
    new_word = reduce_bigram(&new_word);
    //todo: try here https://github.com/finnbear/rustrict/
    //todo-try: try here https://github.com/snguyenthanh/better_profanity/
    // // if new_word.len() < ENGLISH_WORD_LENGTH_THRESHOLD as usize {
    // //     let suggestion = symspell.lookup(&new_word, Verbosity::Top, 1);
    // //     if suggestion.len() != 0 {
    // //         print!("2>");
    // //         return suggestion[0].term.to_owned();
    // //     }
    // // }
    // third: this word may be complex (multiple words and wrong spell), split and try to correct
    // when split misspelling word, the result may contains unknown sequences of 1/2-chars words (not yet impl)
    // ex: he.l.loh.o.w.ar.ey.ou
    // print!("3>");
    symspell
        .word_segmentation(&new_word, ENGLISH_SEGMENTATION_MAX_EDIT_DISTANCE)
        .segmented_string
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
        result_words.push(correct_unknown_word(&word));
    }
    result_words.join(" ")
}
