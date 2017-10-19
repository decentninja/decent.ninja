#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::Template;
use rocket::response::NamedFile;
use rocket::State;

#[macro_use]
extern crate serde_derive;
use std::path::{Path, PathBuf};

mod content;

struct AppState {
    content: content::Content,
}

#[get("/")]
fn index(state: State<AppState>) -> Template {
    Template::render("index", &state.content)
}

// TODO: Fix javascript static thingy to something more dynamic
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    let content = content::read_content().expect("Could not parse content.json");
    rocket::ignite()
        .mount("/", routes![index, files])
        .manage(AppState {
            content: content
        })
        .attach(Template::fairing())
        .launch();
}
