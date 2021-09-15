use bufstream::BufStream;
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

fn handle_req(conn: TcpStream) {
    let mut stream = BufStream::new(&conn);
    if let Err(err) = stream.write(b"> ") {
        println!("Received an error on write! {}", err);
        return
    };
    stream.flush().unwrap();
    let mut request = String::new();
    let size = stream.read_line(&mut request).unwrap();
    if size > 0 {
        print!("Received: {}", request);
        let mut params = request.split_whitespace();
        let command = params.next().unwrap();
        let response = match command {
            "flist" => get_file_list(),
            "md" => make_directory(params.next().unwrap()),
            _ => String::from("Unacceptable command"),
        };
        if let Err(err) = writeln!(stream, "{}", response) {
            println!("Received an error on write! {}", err)
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
