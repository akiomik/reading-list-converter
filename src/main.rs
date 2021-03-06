// Copyright 2021 Akiomi Kamakura
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate clap;

use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

use clap::{App, Arg};
use scraper::Html;

use reading_list_converter::input::InputFormat;
use reading_list_converter::item::Item;
use reading_list_converter::output::OutputFormat;

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
            Arg::with_name("input-format")
                .long("input-format")
                .possible_values(&InputFormat::variants())
                .value_name(&InputFormat::variants().join("|"))
                .takes_value(true)
                .required(true),
            Arg::with_name("output")
                .long("output")
                .value_name("FILE")
                .takes_value(true)
                .required(true),
            Arg::with_name("output-format")
                .long("output-format")
                .takes_value(true)
                .possible_values(&OutputFormat::variants())
                .value_name(&OutputFormat::variants().join("|"))
                .required(true),
        ])
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let input = match fs::read_to_string(input_file) {
        Ok(input) => input,
        Err(e) => panic!("failed to read '{}': {}", input_file, e),
    };

    let input_format =
        value_t!(matches.value_of("input-format"), InputFormat).unwrap_or_else(|e| e.exit());
    let selector = match input_format {
        InputFormat::Pocket => InputFormat::Pocket.selector(),
    };

    let output_file = matches.value_of("output").unwrap();
    let output = match File::create(output_file) {
        Ok(output) => output,
        Err(e) => panic!("failed to create '{}': {}", output_file, e),
    };

    let document = Html::parse_document(&input);
    let mut items: Vec<Item> = Vec::new();
    for el in document.select(&selector) {
        let maybe_url = el.value().attr("href");
        let maybe_title = el.text().next();
        let maybe_added_at = el.value().attr("time_added");
        if let (Some(url), Some(title), Some(added_at)) = (maybe_url, maybe_title, maybe_added_at) {
            items.push(Item {
                url,
                title,
                added_at,
            });
        };
    }

    let output_format =
        value_t!(matches.value_of("output-format"), OutputFormat).unwrap_or_else(|e| e.exit());
    let tmpl = match output_format {
        OutputFormat::Safari => OutputFormat::Safari.template(items),
    };

    match BufWriter::new(output).write(tmpl.as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("failed to convert reading list: {}", e),
    };
}
