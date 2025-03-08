use crate::enigma::utils::ALPHABET;

pub struct Plugboard {
    mapping: Vec<char>,
}

impl Plugboard {
    pub fn new(connections: Vec<(char, char)>) -> Self {
        let mut mapping: Vec<char> = ALPHABET.chars().collect();
        for (a, b) in connections {
            let index_a = ALPHABET.find(a).unwrap();
            let index_b = ALPHABET.find(b).unwrap();
            mapping[index_a] = b;
            mapping[index_b] = a;
        }
        Self { mapping }
    }

    pub fn swap(&self, letter: char) -> char {
        let index = ALPHABET.find(letter).unwrap();
        self.mapping[index]
    }
}