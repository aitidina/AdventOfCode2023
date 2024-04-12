use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::time::{Duration, Instant};

const INPUT_FILE: &str = "day01/input.txt";

fn main() {    
    let start = Instant::now();
    println!("Advent of code 2023 => Puzzle for [Day 01-B]");
    
    let mut file = File::open(INPUT_FILE).unwrap();
    let mut str_file = String::new();
    file.read_to_string(&mut str_file);
    
    
    let mut result: u32 = 0;
    for l in str_file.lines() {
        let mut str_number = String::new();
        let l_reverse: String = l.chars().rev().collect();
        let size_line = l.chars().count();  // Hemen etzitxon problema, baino ondo ziok jakitia len() eztala elementuan rekuentua
        
        for (i, c) in l.char_indices() {
            if c.is_digit(10) {
                str_number.push(c);
                break;
            }
            if i + 2 < size_line {
                match &l[i..i+3] {
                    "one" => {
                        str_number.push('1');
                        break;
                    },
                    "two" => {
                        str_number.push('2');
                        break;
                    },
                    "six" =>  {
                        str_number.push('6');
                        break;
                    },
                    (_) => ()
                }
            }
            if i + 3 < size_line {
                match &l[i..i+4] {
                    "four" => {
                        str_number.push('4');
                        break;
                    },
                    "five" => {
                        str_number.push('5');
                        break;
                    },
                    "nine" => {
                        str_number.push('9');
                        break;
                    },
                    (_) => ()
                }
            }
            if i + 4 < size_line {
                match &l[i..i+5] {
                    "three" => {
                        str_number.push('3');
                        break;
                    },
                    "seven" => {
                        str_number.push('7');
                        break;
                    },
                    "eight" => {
                        str_number.push('8');
                        break;
                    },
                    (_) => ()
                }
            }
        }

        for (i, c) in l_reverse.char_indices() {
            if c.is_digit(10) {
                str_number.push(c);
                break;
            }
            if i + 2 < size_line {
                match &l_reverse[i..i+3] {
                    "eno" => {
                        str_number.push('1');
                        break;
                    },
                    "owt" => {
                        str_number.push('2');
                        break;
                    },
                    "xis" =>  {
                        str_number.push('6');
                        break;
                    },
                    (_) => ()
                }
            }
            if i + 3 < size_line {
                match &l_reverse[i..i+4] {
                    "ruof" => {
                        str_number.push('4');
                        break;
                    },
                    "evif" => {
                        str_number.push('5');
                        break;
                    },
                    "enin" => {
                        str_number.push('9');
                        break;
                    },
                    (_) => ()
                }
            }
            if i + 4 < size_line {
                match &l_reverse[i..i+5] {
                    "eerht" => {
                        str_number.push('3');
                        break;
                    },
                    "neves" => {
                        str_number.push('7');
                        break;
                    },
                    "thgie" => {
                        str_number.push('8');
                        break;
                    },
                    (_) => ()
                }
            }
        }

        // println!("\nString for line is: {}\nReversed string is: {}\nNumber to add is: {}", l, l_reverse, str_number.parse::<u32>().unwrap());

        result += str_number.parse::<u32>().unwrap();
        // std::thread::sleep(std::time::Duration::from_millis(1));
    }
    let duration = start.elapsed();
    println!("This puzzle took {} ms to solve\n", duration.as_millis());

    println!("The result is '{}'", result);
}
