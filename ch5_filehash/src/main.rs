use generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::{env, fs};

enum FileMsg {
    FileHash(OsString, GenericArray<u8, <Sha256 as Digest>::OutputSize>),
    Done
}

fn server(rx: mpsc::Receiver<FileMsg>) {
    for msg in rx {
        match msg {
            FileMsg::FileHash(file,hash) =>
                println!("{:x}  {}", hash, file.to_str().unwrap()),
            FileMsg::Done => return
        }
    }
}

fn read_file(fpath: &Path) -> Result<String, ()> {
    if !fpath.is_dir() {
        match fs::read_to_string(fpath) {
            Ok(content) => Ok(content),
            Err(err) => {
                eprintln!("failed to read file {:?}: {:?}", &fpath, err);
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
        let path = entry.path();
        if let Ok(content) = read_file(&path) {
            let filename = entry.file_name();
            let hash = Sha256::digest(content.as_bytes());
            tx.send(FileMsg::FileHash(filename, hash)).expect("write failed");
        }
    }
    tx.send(FileMsg::Done).expect("failed to send done");
    server_handle.join().expect("thread failed")
}
