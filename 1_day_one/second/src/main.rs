use std::{fs::File, io::Read};

fn main() {
    let mut elf = read_int_from_file();
    elf.sort();
    let elf1 = elf[elf.len() - 1];
    let elf2 = elf[elf.len() - 2];
    let elf3 = elf[elf.len() - 3];
    println!("Elve1: {}, Elve2: {}, Elve3: {}", elf1, elf2, elf3);
    let sum = elf1 + elf2 + elf3;
    println!("Sum: {}", sum);
}
// Function returns a vector of integer sums. Reads from a file seperates 'elves' by empty lines.
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
