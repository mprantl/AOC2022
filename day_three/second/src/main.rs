#![feature(iter_array_chunks)]
use std::{fs::File, io::Read};
fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    let mut result = 0;
    file.read_to_string(&mut contents).expect("Reading failed");
    for [elv_1, elv_2, elv_3] in contents.lines().array_chunks() {
        for i in elv_1.chars() {
            if elv_2.contains(i) && elv_3.contains(i) {
                if i.is_uppercase() {
                    result += i as u32 - 38;
                } else {
                    result += i as u32 - 96;
                }
                break;
            }
        }
    }
    println!("Result: {}", result);
}
