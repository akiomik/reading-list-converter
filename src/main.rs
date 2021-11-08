use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

use askama::Template;
use scraper::{Html, Selector};

struct Item<'a> {
    url: &'a str,
    title: &'a str,
}

#[derive(Template)]
#[template(path = "safari.html")]
struct SafariTemplate<'a> {
    items: Vec<Item<'a>>
}

fn main() {
    let input = match fs::read_to_string("ril_export.html") {
        Ok(input) => input,
        Err(e) => panic!("failed to read file: {}", e),
    };

    let mut file = BufWriter::new(File::create("pocket-to-safari.html").unwrap());

    let document = Html::parse_document(&input);
    let selector = Selector::parse("body > ul > li > a").unwrap();

    let mut items: Vec<Item> = Vec::new();
    for el in document.select(&selector) {
        let maybe_href = el.value().attr("href");
        let maybe_title = el.text().next();
        match (maybe_href, maybe_title) {
            (Some(url), Some(title)) => items.push(Item { url, title }),
            _ => (),
        }
    }

    let tmpl = SafariTemplate { items };
    match file.write(tmpl.render().unwrap().as_bytes()) {
        Ok(_) => println!("done!"),
        Err(e) => panic!("failed to convert reading list: {}", e),
    };
}
