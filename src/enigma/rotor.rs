use crate::enigma::utils::ALPHABET;

pub struct Rotor {
    wiring: String,
    notch: char,
    position: usize,
    ring: usize,
}

impl Rotor {
    pub fn new(wiring: &str, notch: char, initial_position: char, ring_setting: char) -> Self {
        let position = ALPHABET.find(initial_position).expect("Invalid rotor position.");
        let ring = ALPHABET.find(ring_setting).expect("Invalid ring setting.");
        Self {
            wiring: wiring.to_string(),
            notch,
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