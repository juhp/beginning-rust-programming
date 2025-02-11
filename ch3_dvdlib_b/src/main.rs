extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize)]
struct Dvd {
    name: String,
    year: u16,
    cast: String,
    length: u16,
}

fn json_from_str(raw: &str) -> Dvd {
    serde_json::from_str(raw).unwrap()
}

fn str_from_json(dvd: &Dvd) -> String {
    serde_json::to_string(dvd).unwrap()
}

fn dvd_to_file(f: &str, d: Dvd) {
    let file = OpenOptions::new().write(true).open(f).unwrap();
    serde_json::to_writer(file, &d).ok();
}

fn dvd_from_file(f: &str) -> Dvd {
    let file = File::open(f).unwrap();
    let deserialized_json: Dvd = serde_json::from_reader(file).unwrap();
    deserialized_json
}

fn main() {
    let rawdata = r#"
        {
            "name": "La La Land",
            "year": 2016,
            "cast": "Emma Stone, Ryan Gosling",
            "length": 128
        }"#;

    let mut d: Dvd = json_from_str(rawdata);

    let encoded = str_from_json(&d);

    println!("{}", encoded);

    let filename = String::from("file.json");
    dvd_to_file(&filename, d);

    d = dvd_from_file(&filename);
    println!("{}", str_from_json(&d));
}
