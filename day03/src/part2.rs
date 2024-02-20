use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_name: &str = "./input.txt";
    let file: File = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut result: i32 = 0;

    let mut set: HashSet<char> = HashSet::new();
    let mut idx: i32 = 0;
    for line in reader.lines() {
        if let Ok(items) = line {
            if idx == 0 {
                set = items.chars().collect();
            } else {
                set = set
                    .intersection(&items.chars().collect())
                    .cloned()
                    .collect();
            }

            idx = (idx + 1) % 3;
            if idx == 0 {
                let v: Vec<_> = set.clone().into_iter().collect();
                let sym: i32 = v[0] as i32;

                if sym > 96 {
                    result += sym - 96;
                } else {
                    result += sym - 38;
                }
            }
        }
    }

    println!("Result: {}", result);

    Ok(())
}
