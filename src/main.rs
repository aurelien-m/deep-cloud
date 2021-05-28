#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> &'static str {
    "Go away"
}

#[get("/photos")]
fn photos() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("login", context)
}

// #[post("/login")]
// fn login() -> 

fn main() {
    rocket::ignite().mount("/", routes![index, photos])
        .attach(Template::fairing())
        .launch();
}
