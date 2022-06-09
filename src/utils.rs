pub mod assimilations {
    pub const VOWELS: [char; 12] = ['a', 'e', 'i', 'u', 'ā', 'ē', 'ī', 'ū', 'â', 'ê', 'î', 'û'];

    pub fn n_assimilation(first_char: char, second_char: char) -> String {
        /*
            The consonant n nearly always assimilates completely to a following
            consonant; the following consonant is then doubled (i.e., nC1 ! C1C1).
        */
        let mut res = String::new();

        if first_char == 'n' {
            res.push(second_char);
            res.push(second_char);
        } else {
            res.push(first_char);
            res.push(second_char);
        }

        res
    }

    pub fn vowel_assimilation(first_vowel: char, second_vowel: char) -> String {
        let mut res = String::new();

        if !VOWELS.iter().any(|&v| v == first_vowel || v == second_vowel) {
            res.push(first_vowel);
            res.push(second_vowel);
        } else {
            match (first_vowel, second_vowel) {
                // Sequences of long or short e or i followed by long or short a remain uncontracted
                ('e', 'ā') | ('i', 'ā') | ('e', 'a') | ('i', 'a') => {
                    res.push(first_vowel);
                    res.push(second_vowel);
                },
                // an original long e or i that remains as the first vowel in most such sequences is shortened
                ('ē', 'ā') | ('ē', 'a') => {
                    res.push('e');
                    res.push(second_vowel);
                },
                ('ī', 'ā') | ('ī', 'a') => {
                    res.push('i');
                    res.push(second_vowel);
                },
                // A long ā or ē followed by long or short i contracts to ê:
                ('ā', 'i') | ('ā', 'ī') | ('ē', 'i') | ('ē', 'ī') => {
                    res.push('ê');
                },
                /* In all other sequences of contiguous vowels, the vowels contract to a long
                vowel, marked in transcription with a circumflex, that is the quality of the
                original second vowel */
                _ => {
                    if second_vowel == 'a' || second_vowel == 'ā' || second_vowel == 'â' {
                        res.push('â');
                    } else if second_vowel == 'e' || second_vowel == 'ē' || second_vowel == 'ê' {
                        res.push('ê');
                    } else if second_vowel == 'i' || second_vowel == 'ī' || second_vowel == 'î' {
                        res.push('î');
                    } else if second_vowel == 'u' || second_vowel == 'ū' || second_vowel == 'û' {
                        res.push('û');
                    } 
                }
            };
        }

        res
    }
}

pub mod vowel {
    pub fn lengten(vowel: char) -> char {
        match vowel {
            'a' => 'ā',
            'e' => 'ē',
            'i' => 'ī',
            'u' => 'ū',
            'ā' => 'â',
            'ē' => 'ê',
            'ī' => 'î',
            'ū' => 'û',
            _ => panic!("Unexpected phonem: {}", vowel)
        }
    }
}
