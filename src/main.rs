#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(custom_attribute)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::templates::Template;
use rocket::response::NamedFile;
use rocket::State;

#[macro_use]
extern crate serde_derive;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate error_chain;

mod content;

struct AppState {
    content: content::Content,
}

#[get("/")]
fn index(state: State<AppState>) -> Template {
    Template::render("index", &state.content)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    let content = content::read_content("content.json").expect("Could not read content.json.");
    rocket::ignite()
        .mount("/", routes![index, files])
        .manage(AppState {
            content: content
        })
        .attach(Template::fairing())
        .launch();
}
