
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
pub struct Content {
    heading: String,
    short_description: String,
    titlebar: String,
    phone_number: String,
    email: String,
    url: String,
    default_portfolio_id: Option<String>,
    portfolio: Vec<PortfolioPiece>,
    employments: Vec<Employment>,
    educations: Vec<Education>,
    awards: Vec<Award>,
}

error_chain! {
    foreign_links {
        ParseError(serde_json::Error);
        IOError(::std::io::Error);
    }
}

pub fn read_content(path: &str) -> Result<Content> {
    let mut f = File::open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    let mut content: Content = serde_json::from_str(&content)?;
    match content.default_portfolio_id {
        Some(_) => (),
        None => content.default_portfolio_id = Some(content.portfolio[0].id.clone()),
    }
    Ok(content)
}