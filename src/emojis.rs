use std::{collections::HashMap, fs::File, io::BufReader};

pub fn get_emoticons_hashmap() -> HashMap<String, String> {
    let emoticons_filepath = "data/emoticons_emojis/combined_emoji.json";
    let file = File::open(emoticons_filepath).unwrap();
    let reader = BufReader::new(file);
    let emoticons_json: HashMap<String, String> = serde_json::from_reader(reader).unwrap();
    emoticons_json
        .into_iter()
        .filter_map(|(emoticon, emoticon_name)| {
            if emoticon.chars().all(|c| c.is_ascii_alphabetic()) {
                return None;
            }
            Some((emoticon, format!(" {} ", emoticon_name)))
        })
        .collect()
}

pub fn get_unicode_emojis_hashmap() -> HashMap<String, String> {
    let unicode_emojis = emojis::iter()
        .filter_map(|e| {
            if e.unicode_version() > emojis::UnicodeVersion::new(15, 1) {
                return None;
            }
            let e_skin_tones = e.skin_tones();
            if e_skin_tones.is_none() {
                return None;
            }
            Some(e_skin_tones.unwrap().collect::<Vec<_>>())
        })
        .flatten()
        .map(|e| {
            (
                e.as_str().to_owned(),
                format!(" ({}) ", e.name().to_owned()),
            )
        })
        .collect::<HashMap<_, _>>();
    unicode_emojis
}
