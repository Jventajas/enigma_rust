#[macro_use] extern crate rocket;

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
    })
}

#[post("/", data = "<message>")]
fn encrypt(message: Form<Message>) -> Template {


    let alphabet: Vec<char> = ('A'..='Z').collect();
    Template::render("index", context! {
        alphabet: alphabet,
        plaintext: message.plaintext.clone(),
        ciphertext: message.plaintext.clone(),  // Replace with actual encryption logic
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
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, encrypt])
        .mount("/static", FileServer::from("static"))
}








