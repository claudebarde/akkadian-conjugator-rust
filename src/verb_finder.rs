use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerbDataFromJson {
    pub transcription: String,
    pub r#type: String,
    pub stem: String,
    pub verb_adjectival_vowel: char,
    pub theme_vowel: char,
    pub root: Vec<char>,
    pub meaning: Vec<String>
}

#[derive(Debug, Clone)]
pub enum VerbStem {
    GStem,
    GStemWeakIn,
    GStemWeakIII
}

#[derive(Debug, Clone)]
pub enum VerbType {
    Active,
    Adjectival
}

#[derive(Debug, Clone)]
pub struct VerbData {
    pub transcription: String,
    pub r#type: VerbType,
    pub stem: VerbStem,
    pub theme_vowel: char,
    pub verb_adjectival_vowel: char,
    pub root: Vec<char>,
    pub meaning: Vec<String>
}

pub fn find_verb (verb: &String) -> VerbData {
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

    VerbData {
        transcription: verb_data.transcription,
        r#type: 
            match verb_data.r#type.as_str() { 
                | "active" => VerbType::Active,
                | "adjectival" => VerbType::Adjectival,
                | _ => panic!("Unrecognized verb type")
            },
        stem: 
            match verb_data.stem.as_str() {
                | "g-stem" if verb_data.root.len() == 2 => VerbStem::GStemWeakIII,
                | "g-stem" if verb_data.root[0] == 'n' => VerbStem::GStemWeakIn,
                | "g-stem" if verb_data.root[0] != 'n' => VerbStem::GStem,
                | _ => panic!("Unrecognized verb stem")
            },
        theme_vowel: verb_data.theme_vowel,
        verb_adjectival_vowel: verb_data.verb_adjectival_vowel,
        root: verb_data.root,
        meaning: verb_data.meaning
    }
}