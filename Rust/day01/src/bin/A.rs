use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::time::{Duration, Instant};

const INPUT_FILE: &str = "day01/input.txt";

fn main() {    
    let start = Instant::now();
    println!("Advent of code 2023 => Puzzle for [Day 01-A]");
    
    let mut file = File::open(INPUT_FILE).unwrap();
    let mut str_file = String::new();
    file.read_to_string(&mut str_file);
    
    let mut result: u32 = 0;
    for l in str_file.lines() {
        let mut str_number = String::new();
        
        for c in l.chars() {
            if c.is_digit(10) {
                str_number.push(c);
                break;
            }
        }

        for c in l.chars().rev() {
            if c.is_digit(10) {
                str_number.push(c);
                break;
            }
        }

        result += str_number.parse::<u32>().unwrap();
        // std::thread::sleep(std::time::Duration::from_millis(1));
    }
    let duration = start.elapsed();
    println!("This puzzle took {} ms to solve\n", duration.as_millis());

    println!("The result is '{}'", result);
}
