use std::fs::File;
use std::io::Read;
use std::time::Instant;

const INPUT_FILE: &str = "day02/input.txt";
const APP_DESC: &str = "Advent of code 2023 => Puzzle for [Day 02-A]";

const MAX_RED_CUBES:u32 = 12;
const MAX_GREEN_CUBES:u32 = 13;
const MAX_BLUE_CUBES:u32 = 14;

// struct CountColor(u32, str);

fn main() {    
    let start = Instant::now();
    println!("{}", APP_DESC);
    
    let mut file = File::open(INPUT_FILE).unwrap();
    let mut str_file = String::new();
    file.read_to_string(&mut str_file);
    
    
    let (mut result, mut game_count): (u32, u32) = (0, 0);
    for l in str_file.lines() {
        game_count += 1;
        let mut game_possible: bool = true;
        let game_rounds = l.split(':').last().unwrap().split(';');
        
        for r in game_rounds {
            //" 6 red, 1 blue, 3 green"
            let cube_counts_by_color = r.split(',');
            for cube_count in cube_counts_by_color {
                //" 6 red"
                let mut cube_count_pair = cube_count.trim().split(' ');
                let count = cube_count_pair.next().unwrap().parse::<u32>().unwrap();
                let colour = cube_count_pair.next().unwrap();
                
                match colour {
                    "red" => {
                        if count > MAX_RED_CUBES {
                            game_possible = false;
                        }
                    },
                    "green" => {
                        if count > MAX_GREEN_CUBES {
                            game_possible = false;
                        }
                    },
                    "blue" => {
                        if count > MAX_BLUE_CUBES {
                            game_possible = false;
                        }
                    },
                    _ => eprintln!("Not recognised colour found!")
                }
            }
        }

        if game_possible {
            result += game_count;
        }
        // std::thread::sleep(std::time::Duration::from_millis(1));
    }
    let duration = start.elapsed();
    println!("This puzzle took {} ms to solve\n", duration.as_millis());

    println!("The result is '{}'", result);
}
