use crate::enigma::utils::ALPHABET;
use crate::enigma::rotor::Rotor;
use crate::enigma::reflector::Reflector;
use crate::enigma::plugboard::Plugboard;

pub struct EnigmaMachine {
    rotors: Vec<Rotor>,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl EnigmaMachine {
    pub fn new(rotors: Vec<Rotor>, reflector: Reflector, plugboard: Plugboard) -> Self {
        assert_eq!(rotors.len(), 3, "Three rotors are required.");
        Self {
            rotors,
            reflector,
            plugboard,
        }
    }

    fn step_rotors(&mut self) {
        let right_at_notch = self.rotors[2].at_notch();
        let middle_at_notch = self.rotors[1].at_notch();

        if middle_at_notch {
            self.rotors[0].rotate();
        }

        if middle_at_notch || right_at_notch {
            self.rotors[1].rotate();
        }

        self.rotors[2].rotate();
    }

    pub fn process(&mut self, text: &str) -> String {
        text.chars()
            .map(|mut letter| {
                if !ALPHABET.contains(letter) {
                    return letter; // Ignore non-alphabet characters.
                }

                self.step_rotors();
                letter = self.plugboard.swap(letter);

                for rotor in self.rotors.iter().rev() {
                    letter = rotor.encode_forward(letter);
                }

                letter = self.reflector.reflect(letter);

                for rotor in &self.rotors {
                    letter = rotor.encode_backward(letter);
                }

                self.plugboard.swap(letter)
            })
            .collect()
    }
}