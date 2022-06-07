use crate::verb_finder::VerbDataFromJson;

#[derive(Debug)]
pub struct VerbForms {
    pub first_cs: String,
    pub second_ms: String,
    pub second_fs: String,
    pub third_cs: String,
    pub first_cp: String,
    pub second_cp: String,
    pub third_mp: String,
    pub third_fp: String
}

#[derive(Debug)]
pub struct Verb {
    pub verb: String,
    pub preterite: VerbForms,
    pub meaning: Vec<String>,
}

struct PersonAffix<'a> {
    first_cs: &'a str,
    second_ms: &'a str,
    second_fs: &'a str,
    third_cs: &'a str,
    first_cp: &'a str,
    second_cp: &'a str,
    third_mp: &'a str,
    third_fp: &'a str
}

const PERSON_PREFIX: PersonAffix = PersonAffix {
    first_cs: "a",
    second_ms: "ta",
    second_fs: "ta",
    third_cs: "i",
    first_cp: "ni",
    second_cp: "ta",
    third_mp: "i",
    third_fp: "i",
};

const PERSON_SUFFIX: PersonAffix = PersonAffix {
    first_cs: "",
    second_ms: "",
    second_fs: "ī",
    third_cs: "",
    first_cp: "",
    second_cp: "ā",
    third_mp: "ū",
    third_fp: "ā",
};

pub fn conjugate(verb: &String, data: &VerbDataFromJson) -> Verb {
    let preterite = to_preterite(&data).unwrap();

    Verb { 
        verb: verb.clone(),
        preterite: preterite,
        meaning: data.meaning.clone()
    }
}

fn to_preterite(data: &VerbDataFromJson) -> Result<VerbForms, String> {
    match data.stem.as_str() {
        "g-stem" => {
            if data.root.len() == 3 {
                let forms = VerbForms {
                    first_cs: format!("{}{}{}{}{}{}", 
                        PERSON_PREFIX.first_cs, 
                        data.root[0], 
                        data.root[1], 
                        data.theme_vowel, 
                        data.root[2], 
                        PERSON_SUFFIX.first_cs),
                    second_ms: format!("{}{}{}{}{}{}", 
                        PERSON_PREFIX.second_ms, 
                        data.root[0], 
                        data.root[1], 
                        data.theme_vowel, 
                        data.root[2], 
                        PERSON_SUFFIX.second_ms),
                    second_fs: format!("{}{}{}{}{}{}", 
                        PERSON_PREFIX.second_fs, 
                        data.root[0], 
                        data.root[1], 
                        data.theme_vowel, 
                        data.root[2], 
                        PERSON_SUFFIX.second_fs),
                    third_cs: format!("{}{}{}{}{}{}", 
                        PERSON_PREFIX.third_cs, 
                        data.root[0], 
                        data.root[1], 
                        data.theme_vowel, 
                        data.root[2], 
                        PERSON_SUFFIX.third_cs),
                    first_cp: format!("{}{}{}{}{}{}", 
                        PERSON_PREFIX.first_cp, 
                        data.root[0], 
                        data.root[1], 
                        data.theme_vowel, 
                        data.root[2], 
                        PERSON_SUFFIX.first_cp),
                    second_cp: format!("{}{}{}{}{}{}", 
                        PERSON_PREFIX.second_cp, 
                        data.root[0], 
                        data.root[1], 
                        data.theme_vowel, 
                        data.root[2], 
                        PERSON_SUFFIX.second_cp),
                    third_mp: format!("{}{}{}{}{}{}", 
                        PERSON_PREFIX.third_mp, 
                        data.root[0], 
                        data.root[1], 
                        data.theme_vowel, 
                        data.root[2], 
                        PERSON_SUFFIX.third_mp),
                    third_fp: format!("{}{}{}{}{}{}", 
                        PERSON_PREFIX.third_fp, 
                        data.root[0], 
                        data.root[1], 
                        data.theme_vowel, 
                        data.root[2], 
                        PERSON_SUFFIX.third_fp)
                };
                Ok(forms)
            } else {
                Err(String::from("Unexpected non-triliteral root"))
            }
        },
        _ => Err(String::from("Unexpected value for verb stem"))
    }
}