
extern crate serde;
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
pub struct Content  {
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

pub fn read_content() -> Result<Content, serde_json::Error> {
    let mut f = File::open("content.json").expect("file not found");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("something went wrong reading the file");
    serde_json::from_str(&content)
}