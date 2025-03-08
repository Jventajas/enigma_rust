use crate::enigma::utils::ALPHABET;

pub struct Reflector {
    mapping: Vec<char>,
}

impl Reflector {
    pub fn new(wiring: &str) -> Self {
        let mapping: Vec<char> = wiring.chars().collect();
        assert!(mapping.len() == 26, "Reflector wiring must be 26 characters.");
        Self { mapping }
    }

    pub fn reflect(&self, letter: char) -> char {
        let index = ALPHABET.find(letter).unwrap();
        self.mapping[index]
    }
}