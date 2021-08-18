use rand;
use termion;
use std::{env, thread, time};
use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::clear;
use termion::color;

type World = [[bool; 75]; 75];

fn new_world () -> World {
    [[false; 75]; 75]
}

fn main() {
    let mut world = new_world() ;
    let mut generations = 0;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        for i in 0..74 {
            for j in 0..74 {
                world[i][j] = rand::random();
            }
        }
    } else {
        let filename = env::args().nth(1).unwrap();
        world = populate_from_file(filename);
    }

    for _gens in 0..100 {
        let temp = generation(world);
        world = temp;
        generations += 1;
        println!("{}", clear::All);
        displayworld(world);
        println!("{blue}Generation {g}  Population {c}{reset}",
                 blue = color::Fg(color::Blue),
                 g = generations,
                 c = census(world),
                 reset = color::Fg(color::Reset));
        thread::sleep(time::Duration::from_millis(2));
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

    for i in 0..74 {
        for j in 0..74 {
            newworld[i][j] = false;
        }
    }

    for (x,y) in pairs {
        newworld[x][y] = true;
    }
    newworld
}

fn displayworld(world: World)
{
    for i in 0..74 {
        for j in 0..74 {
            if world[i][j]
            {
                print!("{red}*", red = color::Fg(color::Red));
            }
            else
            {
                print!(" ");
            }
        }
        println!("");
    }
}

fn census(world: World) -> u16
{
    let mut count = 0;

    for i in 0..74 {
        for j in 0..74 {
            if world[i][j]
            {
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

    for i in 0..74 {
        for j in 0..74 {
            let mut count = 0;
            if i>0 {
                count = count + cell(world[i-1][j]);
            }
            if i>0 && j>0 {
                count = count + cell(world[i-1][j-1]);
            }
            if i>0 && j<74 {
                count = count + cell(world[i-1][j+1]);
            }
            if i<74 && j>0 {
                count = count + cell(world[i+1][j-1]);
            }
            if i<74 {
                count = count + cell(world[i+1][j]);
            }
            if i<74 && j<74 {
                count = count + cell(world[i+1][j+1]);
            }
            if j>0 {
                count = count + cell(world[i][j-1]);
            }
            if j<74 {
                count = count + cell(world[i][j+1]);
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
