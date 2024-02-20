use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let file_name: &str = "./input.txt";
    let file: File = File::open(file_name)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut path: PathBuf = PathBuf::from("/");

    let mut structure: HashMap<String, u32> = HashMap::new();
    let mut path_str: String = path.clone().into_os_string().into_string().unwrap();
    structure.insert(path_str, 0);

    let mut is_ls: bool = false;

    let mut result: u32 = 0;

    for line in reader.lines() {
        if let Ok(cmd) = line {
            if cmd == "$ ls" {
                is_ls = true;
                continue;
            }
            if cmd.starts_with("$ cd") {
                is_ls = false;
                let (_, dir_name) = cmd.trim().split_at(5);
                if dir_name.trim() == ".." {
                    path.pop();
                } else {
                    path.push(dir_name);
                }
                path_str = path.clone().into_os_string().into_string().unwrap();
                structure.entry(path_str).or_insert(0);
                continue;
            }

            if is_ls == true {
                let tokens: Vec<&str> = cmd.trim().split(' ').collect::<Vec<_>>();
                let size: &str = tokens[0];

                match size.trim().parse::<u32>() {
                    Ok(int_size) => {
                        let mut temp_path = path.clone();

                        while temp_path.parent() != None {
                            path_str = temp_path.clone().into_os_string().into_string().unwrap();
                            if let Some(x) = structure.get_mut(&path_str) {
                                *x += int_size;
                            }
                            temp_path.pop();
                        }
                        *structure.get_mut("/").unwrap() += int_size;
                    }
                    Err(_) => {}
                }
            }
        }
    }
    println!("{:?}", structure);

    for (_, size) in &structure {
        if size <= &100000 {
            result += size
        }
    }
    println!("Result: {}", result);

    Ok(())
}
