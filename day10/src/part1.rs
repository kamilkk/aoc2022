use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn tick(cycle_id: u32, register: i32) -> i32 {
    if cycle_id == 20 || (cycle_id + 20) % 40 == 0 {
        return cycle_id as i32 * register;
    }
    return 0;
}

fn main() -> std::io::Result<()> {
    let file_name: &str = "./input.txt";
    let file: File = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut cycle_id: u32 = 0;
    let mut register: i32 = 1;
    let mut result: i32 = 0;

    for line in reader.lines() {
        if let Ok(instr) = line {
            let tokens: Vec<&str> = instr.split(' ').collect::<Vec<&str>>();
            let cmd: &str = tokens[0];

            if cmd.starts_with("noop") {
                cycle_id += 1;
                result += tick(cycle_id, register);
            } else {
                let new_val = tokens[1].trim().parse::<i32>().unwrap();
                cycle_id += 1;
                result += tick(cycle_id, register);

                cycle_id += 1;
                result += tick(cycle_id, register);
                register += new_val;
            }
        }
    }

    println!("Result: {}", result);

    Ok(())
}
