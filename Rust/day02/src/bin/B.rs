use std::fs::File;
use std::io::Read;
use std::time::Instant;

const INPUT_FILE: &str = "day02/input.txt";
const APP_DESC: &str = "Advent of code 2023 => Puzzle for [Day 02-B]";

fn main() {    
    let start = Instant::now();
    println!("{}", APP_DESC);
    
    let mut file = File::open(INPUT_FILE).unwrap();
    let mut str_file = String::new();
    let _ = file.read_to_string(&mut str_file);
    
    
    let mut result: u32 = 0;
    for l in str_file.lines() {
        let (mut min_red_cubes, mut min_green_cubes, mut min_blue_cubes): (u32, u32, u32) =
            (0, 0, 0);

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
                        if count > min_red_cubes {
                            min_red_cubes = count;
                        }
                    },
                    "green" => {
                        if count > min_green_cubes {
                            min_green_cubes = count;
                        }
                    },
                    "blue" => {
                        if count > min_blue_cubes {
                            min_blue_cubes = count;
                        }
                    },
                    _ => eprintln!("Not recognised colour found!")
                }
            }
        }

        result += min_red_cubes * min_green_cubes * min_blue_cubes;
        // std::thread::sleep(std::time::Duration::from_millis(1));
    }
    let duration = start.elapsed();
    println!("This puzzle took {} ms to solve\n", duration.as_millis());

    println!("The result is '{}'", result);
}
