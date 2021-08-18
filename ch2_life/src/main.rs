use std::{env, thread, time};
use std::fs::File;
use std::io::{BufRead, BufReader, Write, stdout};
use termion::clear;
use termion::color;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::screen::AlternateScreen;

const XSIZE: usize = 160;
const YSIZE: usize = 48;
const ITERATIONS: u64 = 500;
const WAIT: u64 = 100;

type World = [[bool; XSIZE]; YSIZE];

fn new_world () -> World {
    [[false; XSIZE]; YSIZE]
}

fn main() {
    let mut world = new_world() ;
    let mut generations = 0;

    let args = env::args();

    if args.len() < 2 {
        for row in world.iter_mut() {
            for cell in row.iter_mut() {
                *cell = rand::random();
            }
        }
    } else {
        let filename = env::args().nth(1).unwrap();
        world = populate_from_file(filename);
    }

    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut stdin = termion::async_stdin().keys();

    for _gens in 0..ITERATIONS {
        let temp = generation(world);
        world = temp;
        generations += 1;
        write!(screen, "{}\r", clear::All).unwrap();

        for row in world {
            for cell in row {
                write!(screen, "{}", if cell {"o"} else {" "}).unwrap();
            }
            writeln!(screen, "\r").unwrap();
        }

        write!(screen, "{blue}Generation {g}  Population {c}{reset}",
                 blue = color::Fg(color::Blue),
                 g = generations,
                 c = census(world),
                 reset = color::Fg(color::Reset)).unwrap();
        screen.flush().unwrap();
        if let Some(Ok(key)) = stdin.next() {
            match key {
                Key::Char('q') | Key::Esc => break,
                _ =>
                    while stdin.next().is_none() {
                        thread::sleep(time::Duration::from_millis(WAIT));
                    }
            }
        }
        thread::sleep(time::Duration::from_millis(WAIT));
    }

}

fn populate_from_file(filename: String) -> World
{
    let mut newworld = new_world();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut pairs:  Vec<(usize, usize)> = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let mut words = l.split_whitespace();
        let left = words.next().unwrap();
        let right = words.next().unwrap();
        pairs.push((left.parse::<usize>().unwrap(), right.parse::<usize>().unwrap()));
    }

    for (x,y) in pairs {
        newworld[x][y] = true;
    }
    newworld
}

fn census(world: World) -> u16
{
    let mut count = 0;

    for row in world {
        for cell in row {
            if cell {
                count += 1;
            }
        }
    }
    count
}

fn cell (b: bool) -> u8 {
    if b { 1 } else { 0 }
}

fn generation(world: World) -> World
{
    let mut newworld = new_world();

    for i in 0..YSIZE {
        for j in 0..XSIZE {
            let mut count = 0;
            if i>0 {
                count += cell(world[i-1][j]);
            }
            if i>0 && j>0 {
                count += cell(world[i-1][j-1]);
            }
            if i>0 && j<(XSIZE-1) {
                count += cell(world[i-1][j+1]);
            }
            if i<(YSIZE-1) && j>0 {
                count += cell(world[i+1][j-1]);
            }
            if i<(YSIZE-1) {
                count += cell(world[i+1][j]);
            }
            if i<(YSIZE-1) && j<(XSIZE-1) {
                count += cell(world[i+1][j+1]);
            }
            if j>0 {
                count += cell(world[i][j-1]);
            }
            if j<(XSIZE-1) {
                count += cell(world[i][j+1]);
            }

            newworld[i][j] = false;

            if (count <2) && (world[i][j]) {
                newworld[i][j] = false;
            }
            if world[i][j] && (count == 2 || count == 3) {
                newworld[i][j] = true;
            }
            if (!world[i][j]) && (count == 3) {
                newworld[i][j] = true;
            }
        }
    }
    newworld
}
