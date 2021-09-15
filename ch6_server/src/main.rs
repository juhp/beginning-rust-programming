use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn make_directory(param: &str) -> String {
    match fs::create_dir_all(param) {
        Ok(_) => String::from("Success"),
        Err(err) => err.to_string(),
    }
}

fn get_file_list() -> String {
    fs::read_dir(".")
        .unwrap()
        .map(|f| f.unwrap().path().display().to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn handle_req(mut conn: TcpStream) {
    let mut reqbytes = [0; 512];

    if let Err(err) = conn.write(b"> ") {
        println!("Received an error on write! {}", err)
    };
    let requestsize = conn.read(&mut reqbytes);
    let size = requestsize.unwrap();
    let request: String = String::from_utf8(reqbytes[..size].to_vec()).unwrap();
    if size > 0 {
        println!("Received: {}", request);
        let mut params = request.split_whitespace();
        let command = params.next().unwrap();
        let response = match command {
            "flist" => get_file_list(),
            "md" => make_directory(params.next().unwrap()),
            _ => String::from("Unacceptable command"),
        };
        match conn.write(response.as_bytes()) {
            Ok(_) => (),
            Err(err) => println!("Received an error on write! {}", err),
        };
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3333")?;

    for stream in listener.incoming() {
        handle_req(stream?);
    }

    Ok(())
}
