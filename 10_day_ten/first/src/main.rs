use std::{fs::File, io::Read};

fn main() {
    let input = read_input();
    let register: i64 = 1;

    let sum = apply_instrucions(register, &input);
    println!("\n Sum: {}", sum);
}
fn apply_instrucions(mut register: i64, input: &Vec<Vec<String>>) -> i64 {
    let mut sum = 0;
    let mut cycle_count: i64 = 0;
    for line in input {
        let instruction = line[0].as_str();
        match instruction {
            "noop" => {
                cycle_count += 1;
                sum += check_cycle_count(cycle_count, register);
            }
            "addx" => {
                cycle_count += 1;
                sum += check_cycle_count(cycle_count, register);
                cycle_count += 1;
                sum += check_cycle_count(cycle_count, register);

                let amount = line[1].parse::<i64>().unwrap();

                register += amount;
            }
            _ => {
                eprintln!("Error: Invalid instruction: {}", instruction);
            }
        }
    }
    sum
}

fn check_cycle_count(cycle_count: i64, register: i64) -> i64 {
    if cycle_count == 20
        || cycle_count == 60
        || cycle_count == 100
        || cycle_count == 140
        || cycle_count == 180
        || cycle_count == 220
    {
        let product = cycle_count * register;
        product
    } else {
        0
    }
}
fn read_input() -> Vec<Vec<String>> {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    let mut instructions = Vec::new();
    for instruction in contents.lines() {
        instructions.push(instruction.to_string());
    }
    let instructions = instructions
        .iter()
        .map(|x| x.split_whitespace().map(|x| x.to_string()).collect())
        .collect();
    instructions
}
