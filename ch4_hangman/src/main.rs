extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Word {
    answer: String,
    length: usize,
    correct_count: usize,
    representation: String,
}

impl Word {
    fn check_complete(&self) -> bool {
        self.correct_count == self.length
    }

    fn check_for_letter(&mut self, c: char) -> bool {
        let mut count: usize = 0;
        let mut response = String::with_capacity(self.length);
        for (index, letter) in self.answer.chars().enumerate() {
            let repchar = self.representation.chars().nth(index).unwrap();
            if letter == c && repchar != c {
                count += 1;
                response.push(c);
            } else if repchar != '_' {
                response.push(repchar);
            } else {
                response.push('_');
            }
        }
        self.representation = response;
        self.correct_count += count;
        count > 0
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_list(filename: String) -> Vec<String> {
    let mut v = Vec::<String>::new();
    if let Ok(lines) = read_lines(filename) {
        for w in lines {
            let word: String = w.unwrap();
            if word.len() > 4 {
                v.push(word);
            }
        }
    }
    v
}

fn select_word() -> String {
    let mut rng = rand::thread_rng();
    let filename: String = "words.txt".to_string();
    let words: Vec<String> = read_list(filename);
    let word_count = words.len();
    let selection = rng.gen_range(1, word_count);
    let select: String = words[selection].clone();
    select
}

fn main() {
    let body = vec![
        "noose",
        "head",
        "neck",
        "torso",
        "left arm",
        "right arm",
        "right leg",
        "left leg",
        "left foot",
        "right foot",
    ];
    let mut body_iter = body.iter();
    let result = select_word();
    let mut hangman = Word {
        length: result.len(),
        representation: String::from_utf8(vec![b'_'; result.len()]).unwrap(),
        answer: result,
        correct_count: 0,
    };

    let mut body_complete: bool = false;
    while !hangman.check_complete() && !body_complete {
        println!("Provide a letter to guess:");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                // char + newline (getChar would be nice)
                if n == 2 {
                    let letter = input.chars().next().unwrap();
                    if hangman.check_for_letter(letter) {
                        println!(
                            "There is at least one {}, so the word is {}",
                            letter, hangman.representation
                        );
                    } else {
                        let next_part = body_iter.next().unwrap();
                        println!("Incorrect! You are at {}", next_part);
                        if *next_part == "right foot" {
                            body_complete = true;
                        }
                    }
                }
            }
            Err(err) => {
                println!("Unable to read from stdin: {}", err);
            }
        }
    }
    if body_complete {
        println!("You were unsuccessful at guessing {}", &hangman.answer)
    } else {
        println!("Yes! The word was {}", &hangman.answer);
    }
}
