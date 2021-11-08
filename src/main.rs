use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

use askama::Template;
use clap::{App, Arg};
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
    let matches = App::new("reading-list-converter")
        .version("0.1.0")
        .author("Akiomi Kamakura <akiomik@gmail.com>")
        .args(&[
            Arg::with_name("input")
                .long("input")
                .value_name("FILE")
                .takes_value(true)
                .required(true),
            Arg::with_name("output")
                .long("output")
                .value_name("FILE")
                .takes_value(true)
                .required(true),
        ])
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let input = match fs::read_to_string(input_file) {
        Ok(input) => input,
        Err(e) => panic!("failed to read '{}': {}", input_file, e),
    };

    let output_file = matches.value_of("output").unwrap();
    let output = match File::create(output_file) {
        Ok(output) => output,
        Err(e) => panic!("failed to create '{}': {}", output_file, e),
    };

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
    match BufWriter::new(output).write(tmpl.render().unwrap().as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("failed to convert reading list: {}", e),
    };
}
