use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_name: &str = "./input.txt";
    let file: File = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut separator: bool = false;

    for line in reader.lines() {
        if let Ok(items) = line {
            if items.is_empty() {
                separator = true;
                continue;
            }
            if !separator {
                let tokens: Vec<char> = items.chars().collect();
                for idx in (1..items.chars().count()).step_by(4) {
                    let token: char = tokens[idx];
                    if token == '1' {
                        break;
                    }
                    if stacks.len() <= idx / 4 {
                        if token == ' ' {
                            stacks.push(vec![]);
                        } else {
                            stacks.push(vec![token]);
                        }
                    } else {
                        if token != ' ' {
                            stacks[idx / 4].insert(0, token);
                        }
                    }
                }
            } else {
                let tokens: Vec<&str> = items.split(' ').collect();
                let move_size: u32 = tokens[1].parse::<u32>().unwrap();
                let from: usize = tokens[3].parse::<usize>().unwrap() - 1;
                let to: usize = tokens[5].parse::<usize>().unwrap() - 1;

                for _idx in 0..move_size {
                    let temp: char = stacks[from].pop().unwrap();
                    stacks[to].push(temp);
                }
            }
        }
    }

    let mut vector_result: Vec<char> = vec![];
    for idx in 0..stacks.len() {
        let last: usize = stacks[idx].len();
        vector_result.push(stacks[idx][last - 1]);
    }

    let result: String = vector_result.iter().collect();
    println!("Result: {:?}", result);

    Ok(())
}
