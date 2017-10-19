#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::Template;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Link {
    url: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct PortfolioPiece {
    id: String,
    image: String,
    title: String,
    isa: String,
    description: String,
    links: Vec<Link>,
}

#[derive(Serialize, Deserialize)]
struct Employment {
    company: String,
    role: String,
    time: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Education {
    location: String,
    title: String,
    time: String,
}

#[derive(Serialize, Deserialize)]
struct Award {
    name: String,
    from: String,
    time: String,
    description: String,
    links: Vec<Link>
}

#[derive(Serialize, Deserialize)]
struct Content  {
    heading: String,
    short_description: String,
    titlebar: String,
    phone_number: String,
    email: String,
    url: String,
    portfolio: Vec<PortfolioPiece>,
    employments: Vec<Employment>,
    educations: Vec<Education>,
    awards: Vec<Award>,
}

#[get("/")]
fn index() -> Template {
    let mut f = File::open("content.json").expect("file not found");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("something went wrong reading the file");
    let content: Content = serde_json::from_str(&content).unwrap();
    Template::render("index", &content)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
