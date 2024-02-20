use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_name: &str = "./input.txt";
    let file: File = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut result: i32 = 0;

    for line in reader.lines() {
        if let Ok(items) = line {
            let items_length: usize = items.chars().count();
            let (first, second) = items.split_at(items_length / 2);

            let a: HashSet<char> = first.chars().collect();
            let b: HashSet<char> = second.chars().collect();

            let sym: i32 = *a.intersection(&b).collect::<Vec<&char>>()[0] as i32;

            if sym > 96 {
                result += sym - 96;
            } else {
                result += sym - 38;
            }
        }
    }

    println!("Result: {}", result);

    Ok(())
}
