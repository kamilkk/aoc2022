use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_name: &str = "./input.txt";
    let file: File = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut max_calories = 0;
    let mut current_calories = 0;

    for line in reader.lines() {
        if let Ok(val) = line {
            match val.trim().parse::<i32>() {
                Ok(c) => {
                    current_calories += c;
                }
                Err(_) => {
                    max_calories = max(max_calories, current_calories);
                    current_calories = 0;
                }
            }
        }
    }

    println!("Max calories: {}", max_calories);

    Ok(())
}
