use crate::enigma::utils::ALPHABET;

use std::collections::HashMap;

pub fn rotor_configs() -> HashMap<&'static str, (&'static str, char)> {
    let mut configs = HashMap::new();
    configs.insert("I", ("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q'));
    configs.insert("II", ("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E'));
    configs.insert("III", ("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V'));
    configs.insert("IV", ("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J'));
    configs.insert("V", ("VZBRGITYUPSDNHLXAWMJQOFECK", 'Z'));
    configs
}

pub struct Rotor {
    wiring: String,
    notch: char,
    position: usize,
    ring: usize,
}

impl Rotor {
    pub fn new(rotor_number: &str, initial_position: char, ring_setting: char) -> Self {
        // Get the rotor configuration (wiring and notch) for the given rotor number
        let configs = rotor_configs();
        let (wiring, notch) = configs
            .get(rotor_number)
            .expect("Invalid rotor number. Please use 'I', 'II', 'III', 'IV', or 'V'.");

        // Convert initial position and ring setting to numeric indices
        let position = ALPHABET.find(initial_position)
            .expect("Invalid initial position. Must be an uppercase letter A-Z.");
        let ring = ALPHABET.find(ring_setting)
            .expect("Invalid ring setting. Must be an uppercase letter A-Z.");

        // Create and return the Rotor instance
        Self {
            wiring: wiring.to_string(),
            notch: *notch,
            position,
            ring,
        }
    }


    pub fn rotate(&mut self) {
        self.position = (self.position + 1) % 26;
    }

    pub fn at_notch(&self) -> bool {
        ALPHABET.chars().nth(self.position) == Some(self.notch)
    }

    pub fn encode_forward(&self, letter: char) -> char {
        let offset = self.position as isize - self.ring as isize;
        let input_index =
            (ALPHABET.find(letter).unwrap() as isize + offset + 26) as usize % 26;
        let encoded_letter = self.wiring.chars().nth(input_index).unwrap();
        let final_index =
            (ALPHABET.find(encoded_letter).unwrap() as isize - offset + 26) as usize % 26;
        ALPHABET.chars().nth(final_index).unwrap()
    }

    pub fn encode_backward(&self, letter: char) -> char {
        let offset = self.position as isize - self.ring as isize;
        let input_index =
            (ALPHABET.find(letter).unwrap() as isize + offset + 26) as usize % 26;
        let adjusted_letter = ALPHABET.chars().nth(input_index).unwrap();
        let wiring_index =
            self.wiring.find(adjusted_letter).unwrap();
        let final_index =
            (wiring_index as isize - offset + 26) as usize % 26;
        ALPHABET.chars().nth(final_index).unwrap()
    }
}