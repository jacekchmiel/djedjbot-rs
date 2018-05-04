#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate maplit;

use rocket_contrib::Template;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello")]
fn hello() -> Template {
    let ctx = hashmap! {
        "title" => "Enter template!",
        "body" => "Oh Hai!",
    };
    Template::render("hello", &ctx)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, hello])
        .launch();
}