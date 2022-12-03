use std::{fs::File, io::Read};

fn main() {
    let result = read_file();
    println!("Result: {}", result);
}

fn read_file() -> u32 {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    let mut result = 0;
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    for line in contents.lines() {
        let (compartment_1, compartment_2) = line.split_at(line.len() / 2);
        'outer: for c in compartment_1.chars() {
            for d in compartment_2.chars() {
                if c == d {
                    if c.is_uppercase() {
                        result += c as u32 - 38;
                    }
                    if c.is_lowercase() {
                        result += c as u32 - 96;
                    }
                    break 'outer;
                }
            }
        }
    }
    result
}
