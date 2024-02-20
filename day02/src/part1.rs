use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let strategy: HashMap<&str, i32> = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3),
    ]);

    let file_name: &str = "./input.txt";
    let file: File = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut result: i32 = 0;

    for line in reader.lines() {
        if let Ok(val) = line {
            match strategy.get(val.trim()) {
                Some(points) => {
                    result += points;
                }
                None => {
                    println!("Wrong strategy: {}", val)
                }
            }
        }
    }

    println!("Result: {}", result);

    Ok(())
}
