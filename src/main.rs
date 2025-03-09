#[macro_use] extern crate rocket;

use std::collections::HashSet;

use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer};
use rocket::form::Form;
use rocket::futures::stream::Collect;

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
    let plugboard = message.plugboard.clone();

    // Function to validate plugboard settings
    fn is_plugboard_valid(plugboard: &str) -> Result<(), String> {
        let mut used_letters = HashSet::new();
        let pairs: Vec<&str> = plugboard.split_whitespace().collect();

        for pair in pairs {
            let chars: Vec<char> = pair.chars().collect();
            if chars.len() != 2 {
                return Err(format!("Invalid pair: '{}'", pair));
            }
            for &ch in &chars {
                if !ch.is_ascii_uppercase() || used_letters.contains(&ch) {
                    return Err(format!("Invalid or duplicate letter: '{}'", ch));
                }
                used_letters.insert(ch);
            }
        }
        Ok(())
    }

    // Validate the plugboard
    let validation_result = is_plugboard_valid(&plugboard);

    match validation_result {
        Ok(_) => Template::render("index", context! {
            alphabet: alphabet,
            plaintext: message.plaintext.clone(),
            ciphertext: message.plaintext.clone(),  // Use the plaintext as a placeholder
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
        }),
        Err(error_message) => Template::render("index", context! {
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
            error: error_message,  // Pass the error to the template
        }),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, encrypt])
        .mount("/static", FileServer::from("static"))
}








