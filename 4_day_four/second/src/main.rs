use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let mut result = 0;
    for line in contents.lines() {
        let section_id: Vec<u32> = line
            .split(&[',', '-'])
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        if section_id[1] < section_id[2] || section_id[3] < section_id[0] {
        } else {
            result += 1;
        }
    }
    println!("Result: {}", result);
}
