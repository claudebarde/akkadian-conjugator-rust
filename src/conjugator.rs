use crate::verb_finder::{VerbData, VerbStem, VerbType};
use crate::utils::assimilations::{n_assimilation, vowel_assimilation};
use crate::utils::vowel;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Verb {
    pub verb: String,
    pub preterite: VerbForms,
    pub adjective: [String; 2],
    pub data: VerbData,
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

const PERSON_PREFIX_E: PersonAffix = PersonAffix {
    first_cs: "e",
    second_ms: "te",
    second_fs: "te",
    third_cs: "i",
    first_cp: "ni",
    second_cp: "te",
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

pub fn conjugate(verb: &String, data: &VerbData) -> Verb {
    let preterite = to_preterite(&data).unwrap();
    let adjective = to_adjective(&data);

    Verb { 
        verb: verb.clone(),
        preterite: preterite,
        adjective: adjective,
        data: data.clone()
    }
}

fn to_preterite(data: &VerbData) -> Result<VerbForms, String> {
    fn build_triliteral_preterit(root: &Vec<char>, prefix: &str, suffix: &str, theme_vowel: char) -> String {
        format!("{}{}{}{}{}{}", prefix, root[0], root[1], theme_vowel, root[2], suffix)
    }

    fn build_biliteral_preterit(root: &Vec<char>, prefix: &str, suffix: &str, theme_vowel: char) -> String {
        match suffix.chars().nth(0) {
            None => {
                // if no suffix
                format!("{}{}{}{}", prefix, root[0], root[1], theme_vowel)
            },
            Some (c) => {
                // if suffix
                format!("{}{}{}{}", prefix, root[0], root[1], vowel_assimilation(theme_vowel, c))
            }
        }
    }

    match &data.stem {
        VerbStem::GStem => {
            if data.root.len() == 3 {
                let forms = VerbForms {
                    first_cs: build_triliteral_preterit(&data.root, PERSON_PREFIX.first_cs, PERSON_SUFFIX.first_cs, data.theme_vowel),
                    second_ms: build_triliteral_preterit(&data.root, PERSON_PREFIX.second_ms, PERSON_SUFFIX.second_ms, data.theme_vowel),
                    second_fs: build_triliteral_preterit(&data.root, PERSON_PREFIX.second_fs, PERSON_SUFFIX.second_fs, data.theme_vowel),
                    third_cs: build_triliteral_preterit(&data.root, PERSON_PREFIX.third_cs, PERSON_SUFFIX.third_cs, data.theme_vowel),
                    first_cp: build_triliteral_preterit(&data.root, PERSON_PREFIX.first_cp, PERSON_SUFFIX.first_cp, data.theme_vowel),
                    second_cp: build_triliteral_preterit(&data.root, PERSON_PREFIX.second_cp, PERSON_SUFFIX.second_cp, data.theme_vowel),
                    third_mp: build_triliteral_preterit(&data.root, PERSON_PREFIX.third_mp, PERSON_SUFFIX.third_mp, data.theme_vowel),
                    third_fp: build_triliteral_preterit(&data.root, PERSON_PREFIX.third_fp, PERSON_SUFFIX.third_fp, data.theme_vowel)
                };
                Ok(forms)
            } else {
                Err(String::from("Unexpected non-triliteral root"))
            }
        },
        VerbStem::GStemWeakIn => {
            if data.root.len() == 3 {
                // R1 = 'n' assimilates to following consonant
                let root: Vec<char> = vec![data.root[1], data.root[1], data.root[2]];
                let forms = VerbForms {
                    first_cs: build_triliteral_preterit(&root, PERSON_PREFIX.first_cs, PERSON_SUFFIX.first_cs, data.theme_vowel),
                    second_ms: build_triliteral_preterit(&root, PERSON_PREFIX.second_ms, PERSON_SUFFIX.second_ms, data.theme_vowel),
                    second_fs: build_triliteral_preterit(&root, PERSON_PREFIX.second_fs, PERSON_SUFFIX.second_fs, data.theme_vowel),
                    third_cs: build_triliteral_preterit(&root, PERSON_PREFIX.third_cs, PERSON_SUFFIX.third_cs, data.theme_vowel),
                    first_cp: build_triliteral_preterit(&root, PERSON_PREFIX.first_cp, PERSON_SUFFIX.first_cp, data.theme_vowel),
                    second_cp: build_triliteral_preterit(&root, PERSON_PREFIX.second_cp, PERSON_SUFFIX.second_cp, data.theme_vowel),
                    third_mp: build_triliteral_preterit(&root, PERSON_PREFIX.third_mp, PERSON_SUFFIX.third_mp, data.theme_vowel),
                    third_fp: build_triliteral_preterit(&root, PERSON_PREFIX.third_fp, PERSON_SUFFIX.third_fp, data.theme_vowel)
                };
                Ok(forms)
            } else {
                Err(String::from("Unexpected non-triliteral root"))
            }
        },
        VerbStem::GStemWeakIII => {
            if data.root.len() == 2 {
                // loss of R3
                let prefix = if data.theme_vowel == 'e' { PERSON_PREFIX_E } else { PERSON_PREFIX };
                let forms = VerbForms {
                    first_cs: build_biliteral_preterit(&data.root, prefix.first_cs, PERSON_SUFFIX.first_cs, data.theme_vowel),
                    second_ms: build_biliteral_preterit(&data.root, prefix.second_ms, PERSON_SUFFIX.second_ms, data.theme_vowel),
                    second_fs: build_biliteral_preterit(&data.root, prefix.second_fs, PERSON_SUFFIX.second_fs, data.theme_vowel),
                    third_cs: build_biliteral_preterit(&data.root, prefix.third_cs, PERSON_SUFFIX.third_cs, data.theme_vowel),
                    first_cp: build_biliteral_preterit(&data.root, prefix.first_cp, PERSON_SUFFIX.first_cp, data.theme_vowel),
                    second_cp: build_biliteral_preterit(&data.root, prefix.second_cp, PERSON_SUFFIX.second_cp, data.theme_vowel),
                    third_mp: build_biliteral_preterit(&data.root, prefix.third_mp, PERSON_SUFFIX.third_mp, data.theme_vowel),
                    third_fp: build_biliteral_preterit(&data.root, prefix.third_fp, PERSON_SUFFIX.third_fp, data.theme_vowel)
                };
                Ok(forms)
            } else {
                Err(String::from("Unexpected non-biliteral root"))
            }
        }
    }
}

fn to_adjective(data: &VerbData) -> [String; 2] {
    if data.root.len() == 3 {
        match data.r#type {
            VerbType::Adjectival if data.root[1] == data.root[2] =>
                // adjectival verbs with same R2 and R3
                [
                    format!("{}a{}{}um", data.root[0], data.root[1], data.root[2]), 
                    format!("{}a{}{}atum", data.root[0], data.root[1], data.root[2])
                ],
            VerbType::Active | VerbType::Adjectival if data.root[2] == 'd' || data.root[2] == 'ṭ' => {
                // 'd' and 'ṭ' assimilations
                [
                    format!("{}a{}{}um", data.root[0], data.root[1], data.root[2]), 
                    format!("{}a{}{}{}tum", data.root[0], data.root[1], data.verb_adjectival_vowel, 't')
                ]
            },
            VerbType::Active | VerbType::Adjectival if data.root[2] == 's' || data.root[2] == 'ṣ' || data.root[2] == 'z' => {
                // 's', 'ṣ' and 'z' assimilations
                [
                    format!("{}a{}{}um", data.root[0], data.root[1], data.root[2]), 
                    format!("{}a{}{}{}tum", data.root[0], data.root[1], data.verb_adjectival_vowel, 'š')
                ]
            },
            VerbType::Active | VerbType::Adjectival => {
                // general case
                    let r1 = data.root[0];
                    let r2 = data.root[1];
                    let r3 = data.root[2];
                    // possible n assimilation
                    let masculine = format!("{}a{}{}um", r1, r2, r3);
                    let feminine = format!("{}a{}{}{}um", r1, r2, data.verb_adjectival_vowel, n_assimilation(r3, 't'));
    
                    [
                        masculine, 
                        feminine
                    ]
            }
        }
    } else if data.root.len() == 2 {
        let r1 = data.root[0];
        let r2 = data.root[1];
        let masculine = format!("{}a{}ûm", r1, r2);
        let feminine = format!("{}a{}{}tum", r1, r2, vowel::lengten(data.verb_adjectival_vowel));

        [
            masculine, 
            feminine
        ]
    } else {
        panic!("Unexpected number of consonants in the verb root")
    }
}