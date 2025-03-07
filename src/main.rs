#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer};
use rocket::form::Form;

#[derive(FromForm)]
struct Message {
    plaintext: String,
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
    Template::render("index", context! { alphabet: alphabet, name: "Rocket User" })
}

#[post("/", data = "<message>")]
fn encrypt(message: Form<Message>) -> Template {
    let alphabet: Vec<char> = ('A'..='Z').collect();
    Template::render("index", context! {
        alphabet: alphabet,
        name: "Rocket User",
        ciphertext: message.plaintext.clone(),
        plaintext: message.plaintext.clone(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, encrypt])
        .mount("/static", FileServer::from("static"))
}








