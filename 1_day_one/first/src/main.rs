use std::{fs::File, io::Read};

fn main() {
    let elf = read_int_from_file();
    // Print all elfs
    /*
    for i in &elf {
        println!("{} ", i);
    }
    */
    let elf_counter = &elf.len();
    println!("Elve counter: {}", elf_counter);
    let max_calories = elf.iter().max().unwrap();
    let most_bussiest_elf = &elf.iter().position(|&r| r == *max_calories).unwrap();
    print!(
        "Max calories: {}   Bussiest Elve: {}",
        max_calories, most_bussiest_elf
    );
}
// Function returns a vector of integer sums. Reads from a file seperated by empty lines.
fn read_int_from_file() -> Vec<i32> {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let lines = contents.lines();
    let mut elf_vec: Vec<i32> = Vec::new();
    let mut num: i32 = 0;
    for line in lines {
        if line != "" {
            num += line.parse::<i32>().unwrap();
        } else {
            elf_vec.push(num);
            num = 0;
        }
    }
    elf_vec
}
