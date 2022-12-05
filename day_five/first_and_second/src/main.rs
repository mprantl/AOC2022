use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Clone)]
struct Instruction {
    amount: u32,
    origin_stack: u32,
    destination_stack: u32,
}
fn main() {
    //create crate stacks and instructions
    let mut crate_stacks = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    //parse input
    (crate_stacks, instructions) = parse_input(crate_stacks, instructions);
    //apply instructions
    let crate_stacks_part1 = apply_instructions_part1(crate_stacks.clone(), instructions.clone());
    let crate_stacks_part2 = apply_instructions_part2(crate_stacks.clone(), instructions.clone());
    //print the final state of the stacks
    println!("\nPart 1:");
    let result_1 = print_result(crate_stacks_part1);
    print!("Result: {}", result_1);
    println!("\nPart 2:");
    let result_2 = print_result(crate_stacks_part2);
    print!("Result: {}", result_2);
}

fn print_result(crate_stack: HashMap<u32, Vec<char>>) -> String {
    let mut keys: Vec<_> = crate_stack.keys().collect();
    keys.sort();
    let mut result = String::new();
    for k in keys {
        let values = crate_stack.get(k).unwrap();
        println!("{}: {:?}", k, values);
        let char = values.clone().pop().unwrap();
        result.push(char);
    }
    result
}

fn apply_instructions_part1(
    mut crate_stacks: HashMap<u32, Vec<char>>,
    instructions: Vec<Instruction>,
) -> HashMap<u32, Vec<char>> {
    for mut instruction in instructions {
        while instruction.amount > 0 {
            let top = crate_stacks
                .entry(instruction.origin_stack - 1)
                .or_insert(Vec::new())
                .pop();
            match top {
                Some(x) => {
                    crate_stacks
                        .entry(instruction.destination_stack - 1)
                        .or_insert(Vec::new())
                        .push(x);
                }
                None => {
                    println!("Error: Stack is empty");
                }
            }
            instruction.amount -= 1;
        }
    }
    crate_stacks
}

fn apply_instructions_part2(
    mut crate_stacks: HashMap<u32, Vec<char>>,
    instructions: Vec<Instruction>,
) -> HashMap<u32, Vec<char>> {
    for instruction in instructions {
        let top = crate_stacks
            .entry(instruction.origin_stack - 1)
            .or_insert(Vec::new());

        let at = top.len() - instruction.amount as usize;
        let mut to_move = top.split_off(at);
        crate_stacks
            .entry(instruction.destination_stack - 1)
            .or_insert(Vec::new())
            .append(&mut to_move);
    }
    crate_stacks
}

fn parse_input(
    mut crate_stacks: HashMap<u32, Vec<char>>,
    mut instructions: Vec<Instruction>,
) -> (HashMap<u32, Vec<char>>, Vec<Instruction>) {
    //read input
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let mut blank_line: usize = 0;

    //get the crate stacks
    for line in contents.lines().enumerate() {
        if line.1.is_empty() {
            blank_line = line.0;
            break;
        } else {
            let chars = line.1.chars();
            let mut key;
            for c in chars.enumerate() {
                if c.1.is_uppercase() {
                    key = ((c.0 as u32) - 1) / 4;
                    crate_stacks.entry(key).or_insert(Vec::new()).insert(0, c.1);
                }
            }
        }
    }
    //get the instructions
    for line in contents.lines().skip(blank_line + 1) {
        let iter = line.split_whitespace();
        let (mut amount, mut origin_stack, mut destination_stack) = (0, 0, 0);
        for i in iter.enumerate() {
            if i.0 == 1 {
                amount = i.1.parse::<u32>().unwrap();
            }
            if i.0 == 3 {
                origin_stack = i.1.parse::<u32>().unwrap();
            }
            if i.0 == 5 {
                destination_stack = i.1.parse::<u32>().unwrap();
            }
        }
        instructions.push(Instruction {
            amount,
            origin_stack,
            destination_stack,
        });
    }
    (crate_stacks, instructions)
}
