use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn tick(cycle_id: u32, register: i32, beam_position: i32) {
    if beam_position >= register - 1 && beam_position <= register + 1 {
        print!("#")
    } else {
        print!(".")
    }

    if cycle_id % 40 == 0 {
        print!("\n");
    }
}

fn main() -> std::io::Result<()> {
    let file_name: &str = "./input.txt";
    let file: File = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut cycle_id: u32 = 0;
    let mut register: i32 = 1;
    let mut beam_position: i32 = -1;

    for line in reader.lines() {
        if let Ok(instr) = line {
            let tokens: Vec<&str> = instr.split(' ').collect::<Vec<&str>>();
            let cmd: &str = tokens[0];

            if cmd.starts_with("noop") {
                cycle_id += 1;
                beam_position = (beam_position + 1) % 40;
                tick(cycle_id, register, beam_position);
            } else {
                let new_val = tokens[1].trim().parse::<i32>().unwrap();
                cycle_id += 1;
                beam_position = (beam_position + 1) % 40;
                tick(cycle_id, register, beam_position);

                cycle_id += 1;
                beam_position = (beam_position + 1) % 40;
                tick(cycle_id, register, beam_position);
                register += new_val;
            }
        }
    }

    Ok(())
}
