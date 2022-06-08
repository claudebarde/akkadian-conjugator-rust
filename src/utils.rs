pub mod assimilations {
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
}
