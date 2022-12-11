use std::{fs::File, io::Read};
#[derive(Debug, Clone)]
struct Sprite {
    middle_pixel: i64,
    left_pixel: i64,
    right_pixel: i64,
}

impl Sprite {
    fn new() -> Sprite {
        Sprite {
            middle_pixel: 0,
            left_pixel: 0,
            right_pixel: 0,
        }
    }
    fn set_range(&mut self, center_pixel: i64) {
        self.middle_pixel = center_pixel;
        self.left_pixel = center_pixel - 1;
        self.right_pixel = center_pixel + 1;
    }
}

fn main() {
    let input = read_input();
    let register: i64 = 1;
    apply_instrucions(register, &input);
}
fn apply_instrucions(mut register: i64, input: &Vec<Vec<String>>) {
    let mut cycle_count: i64 = 0;
    let mut sprite = Sprite::new();
    for line in input {
        let instruction = line[0].as_str();
        match instruction {
            "noop" => {
                cycle_count += 1;
                draw_crt(cycle_count, sprite.clone());
            }
            "addx" => {
                cycle_count += 1;
                draw_crt(cycle_count, sprite.clone());
                cycle_count += 1;
                draw_crt(cycle_count, sprite.clone());

                let amount = line[1].parse::<i64>().unwrap();

                register += amount;
                sprite.set_range(register);
            }
            _ => {
                eprintln!("Error: Invalid instruction: {}", instruction);
            }
        }
    }
}

fn draw_crt(cycle_count: i64, sprite: Sprite) {
    let pointer = (cycle_count - 1) % 40;
    if pointer >= sprite.left_pixel && pointer <= sprite.right_pixel {
        print!("#");
    } else {
        print!(".");
    }
    if cycle_count % 40 == 0 {
        println!("");
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
