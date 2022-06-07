use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct VerbDataFromJson {
    pub transcription: String,
    pub stem: String,
    pub theme_vowel: char,
    pub root: Vec<char>,
    pub meaning: String
}

pub fn find_verb (verb: &String) -> VerbDataFromJson {
    // gets the first char of the verb
    let first_char = verb.chars().nth(0).unwrap();
    // finds the correcponding JSON file
    let path = format!("src/verbs/{}.json", first_char);
    let verb_json_data = 
        match fs::read_to_string(path) {
            Ok (val) => val,
            Err (_) => panic!("Couldn't find dictionary for {}", first_char)
        };
    // parses the JSON result
    let dict: Value = serde_json::from_str(&*verb_json_data).unwrap();
    // parses the verb data
    let verb_data: VerbDataFromJson = 
        match dict.get(verb) {
            None => panic!("Couldn't find the verb {}", verb),
            Some (v) => serde_json::from_str(&Value::to_string(&v)).unwrap()
        };
    verb_data
}