use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::sync::mpsc;
use std::thread;
use std::{env, fs};

enum FileMsg {
    FileHash(OsString, String),
    Done
}

fn server(rx: mpsc::Receiver<FileMsg>) {
    for msg in rx {
        match msg {
            FileMsg::FileHash(file,hash) =>
                println!("{}  {}", hash, file.to_str().unwrap()),
            FileMsg::Done => return
        }
    }
}

fn read_file(filename: &fs::DirEntry) -> Result<String, ()> {
    let fpath = filename.path();
    if !fpath.is_dir() {
        match fs::read_to_string(fpath) {
            Ok(content) => Ok(content),
            Err(err) => {
                eprintln!("failed to read file {:?}: {:?}", filename.path(), err);
                Err(())
            }
        }
    } else {
        Err(())
    }
}

fn main() {
    let current_dir = String::from(env::current_dir().unwrap().to_str().unwrap());
    let (tx, rx) = mpsc::channel();
    let server_handle = thread::spawn(|| server(rx));
    for entry in fs::read_dir(&current_dir).expect("could not read dir") {
        let entry = entry.unwrap();
        if let Ok(content) = read_file(&entry) {
            let path = entry.file_name();
            let hash = format!("{:x}", Sha256::digest(content.as_bytes()));
            tx.send(FileMsg::FileHash(path, hash)).expect("write failed");
        }
    }
    tx.send(FileMsg::Done).expect("failed to send done");
    server_handle.join().expect("thread failed")
}
