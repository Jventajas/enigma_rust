#[macro_use] extern crate rocket;
mod enigma;

use std::collections::HashSet;

use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer};
use rocket::form::Form;
use rocket::futures::stream::Collect;

use enigma::enigma_machine::EnigmaMachine;
use enigma::rotor::Rotor;
use enigma::reflector::Reflector;
use enigma::plugboard::Plugboard;

#[derive(FromForm)]
struct Message {
    plaintext: String,
    ciphertext: String,
    left_rotor: String,
    left_initial_position: char,
    left_ring_setting: char,
    center_rotor: String,
    center_initial_position: char,
    center_ring_setting: char,
    right_rotor: String,
    right_initial_position: char,
    right_ring_setting: char,
    reflector: String,
    plugboard: String,
}


#[get("/")]
fn index() -> Template {
    let alphabet: Vec<char> = ('A'..='Z').collect();
    Template::render("index", context! {
        alphabet: alphabet,
        plugboard: "AB CD EF",
        left_rotor: "I",
        center_rotor: "II",
        right_rotor: "III",

    })
}

#[post("/", data = "<message>")]
fn encrypt(message: Form<Message>) -> Template {
    let alphabet: Vec<char> = ('A'..='Z').collect();
    let plugboard_str = message.plugboard.clone();

    match parse_plugboard_string(&plugboard_str) {
        Ok(pairs) => {
            let left_rotor = Rotor::new(
                &message.left_rotor,
                message.left_initial_position,
                message.left_ring_setting,
            );
            let center_rotor = Rotor::new(
                &message.center_rotor,
                message.center_initial_position,
                message.center_ring_setting,
            );
            let right_rotor = Rotor::new(
                &message.right_rotor,
                message.right_initial_position,
                message.right_ring_setting,
            );

            let reflector = Reflector::new(&message.reflector);
            let plugboard = Plugboard::new(pairs);

            // Initialize the Enigma machine
            let mut enigma = EnigmaMachine::new(
                vec![left_rotor, center_rotor, right_rotor],
                reflector,
                plugboard,
            );

            // Encrypt the plaintext
            let ciphertext = enigma.process(&message.plaintext.to_uppercase());

            // Render the template with ciphertext
            Template::render("index", context! {
                alphabet: alphabet,
                plaintext: message.plaintext.clone(),
                ciphertext: ciphertext,  // Use the encrypted text
                left_rotor: message.left_rotor.clone(),
                left_initial_position: message.left_initial_position,
                left_ring_setting: message.left_ring_setting,
                center_rotor: message.center_rotor.clone(),
                center_initial_position: message.center_initial_position,
                center_ring_setting: message.center_ring_setting,
                right_rotor: message.right_rotor.clone(),
                right_initial_position: message.right_initial_position,
                right_ring_setting: message.right_ring_setting,
                reflector: message.reflector.clone(),
                plugboard: message.plugboard.clone(),
            })
        },
        Err(e) => Template::render("index", context! {
            alphabet: alphabet,
            plaintext: message.plaintext.clone(),
            ciphertext: "",  // No ciphertext on error
            left_rotor: message.left_rotor.clone(),
            left_initial_position: message.left_initial_position,
            left_ring_setting: message.left_ring_setting,
            center_rotor: message.center_rotor.clone(),
            center_initial_position: message.center_initial_position,
            center_ring_setting: message.center_ring_setting,
            right_rotor: message.right_rotor.clone(),
            right_initial_position: message.right_initial_position,
            right_ring_setting: message.right_ring_setting,
            reflector: message.reflector.clone(),
            plugboard: message.plugboard.clone(),
            error: e,  // Pass the error to the template
        }),
    }
}

fn parse_plugboard_string(plugboard: &str) -> Result<Vec<(char, char)>, String> {
    let mut connections = Vec::new();
    let pairs: Vec<&str> = plugboard.split_whitespace().collect();

    for pair in pairs {
        if pair.len() != 2 {
            return Err(format!("Invalid plugboard pair: {}", pair));
        }
        let chars: Vec<char> = pair.chars().collect();
        let a = chars[0];
        let b = chars[1];

        // Ensure both characters are uppercase alphabet letters
        if !a.is_ascii_alphabetic() || !b.is_ascii_alphabetic() || !a.is_uppercase() || !b.is_uppercase() {
            return Err(format!("Invalid characters in plugboard pair: {}", pair));
        }

        // Ensure no duplicate connections
        if connections.iter().any(|(x, y)| *x == a || *y == a || *x == b || *y == b) {
            return Err(format!(
                "Plugboard contains duplicate or conflicting connection: {}",
                pair
            ));
        }

        connections.push((a, b));
    }
    Ok(connections)
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, encrypt])
        .mount("/static", FileServer::from("static"))
}








