#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer};

#[get("/")]
fn index() -> Template {
    let alphabet: Vec<char> = ('A'..='Z').collect();
    Template::render("index", context! { alphabet: alphabet, name: "Rocket User" })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/static", FileServer::from("static"))
}
