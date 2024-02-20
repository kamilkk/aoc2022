use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_name = "./input.txt";
    let file: File = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut max_calories: VecDeque<u32> = [0, 0, 0].into();
    let mut current_calories = 0;

    for line in reader.lines() {
        if let Ok(val) = line {
            match val.trim().parse::<u32>() {
                Ok(c) => {
                    current_calories += c;
                }
                Err(_) => {
                    if current_calories > max_calories[2] {
                        max_calories.push_back(current_calories);
                        let mut _tmp = max_calories.pop_front();
                    } else if current_calories > max_calories[1] {
                        max_calories.insert(2, current_calories);
                        let mut _tmp = max_calories.pop_front();
                    } else if current_calories > max_calories[0] {
                        max_calories[0] = current_calories
                    }
                    current_calories = 0
                }
            }
        }
    }

    let result: u32 = max_calories.iter().sum();
    println!("Max calories: {}", result);

    Ok(())
}
