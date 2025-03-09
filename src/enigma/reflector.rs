use crate::enigma::utils::ALPHABET;

use std::collections::HashMap;

pub fn reflector_configs() -> HashMap<&'static str, &'static str> {
    let mut configs = HashMap::new();
    configs.insert("A", "EJMZALYXVBWFCRQUONTSPIKHGD");
    configs.insert("B", "YRUHQSLDPXNGOKMIEBFZCWVJAT");
    configs.insert("C", "FVPJIAOYEDRZXWGCTKUQSBNMHL");
    configs
}

pub struct Reflector {
    mapping: Vec<char>,
}

impl Reflector {
    pub fn new(reflector_type: &str) -> Self {
        // Fetch the reflector configuration
        let configs = reflector_configs();
        let wiring = configs
            .get(reflector_type)
            .expect("Invalid reflector type. Please use 'A', 'B', or 'C'.");

        // Convert the wiring into the Reflector structure
        let mapping: Vec<char> = wiring.chars().collect();
        assert_eq!(mapping.len(), 26, "Reflector wiring must be 26 characters.");
        Self { mapping }
    }

    pub fn reflect(&self, letter: char) -> char {
        let index = ALPHABET.find(letter).unwrap();
        self.mapping[index]
    }
}