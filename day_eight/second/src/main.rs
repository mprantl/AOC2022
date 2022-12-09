use std::{fs::File, io::Read};
#[derive(Debug, Clone, Copy)]
struct Tree {
    size: u32,
    scenic_score: u32,
    visible_left: u32,
    visible_right: u32,
    visible_above: u32,
    visible_below: u32,
}
impl Tree {
    fn new(size: u32) -> Tree {
        Tree {
            size,
            scenic_score: 0,
            visible_left: 0,
            visible_right: 0,
            visible_above: 0,
            visible_below: 0,
        }
    }
    fn get_scenic_score(&self) -> u32 {
        self.scenic_score
    }
    fn increment_left(&mut self) {
        self.visible_left += 1;
    }
    fn increment_right(&mut self) {
        self.visible_right += 1;
    }
    fn increment_above(&mut self) {
        self.visible_above += 1;
    }
    fn increment_below(&mut self) {
        self.visible_below += 1;
    }
    fn set_scenic_score(&mut self) {
        self.scenic_score =
            self.visible_left * self.visible_right * self.visible_above * self.visible_below;
    }
}
fn main() {
    let mut vec = read_file();
    vec = visible_tree_amount(vec);
    let mut max_score = 0;
    for row in vec.iter_mut() {
        for tree in row.iter_mut() {
            tree.set_scenic_score();
            if tree.get_scenic_score() > max_score {
                max_score = tree.get_scenic_score();
            }
        }
    }
    println!("\n Best score: {}", max_score);
}

fn read_file() -> Vec<Vec<Tree>> {
    let mut file = File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let mut vec: Vec<Vec<Tree>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<Tree> = Vec::new();
        for c in line.chars() {
            let size = c.to_digit(10).unwrap();
            row.push(Tree::new(size));
        }
        vec.push(row);
    }
    vec
}
fn visible_tree_amount(mut vec: Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
    for row in 0..vec.len() {
        for column in 0..vec[row].len() {
            //let mut tree = vec[row][column];
            // Check the trees to the left
            for i in (0..column).rev() {
                if vec[row][i].size >= vec[row][column].size {
                    vec[row][column].increment_left();
                    break;
                } else {
                    vec[row][column].increment_left();
                }
            }
            // Check the trees to the right
            for i in column + 1..vec[row].len() {
                if vec[row][i].size >= vec[row][column].size {
                    vec[row][column].increment_right();
                    break;
                } else {
                    vec[row][column].increment_right();
                }
            }
            // Check the trees above
            for i in (0..row).rev() {
                if vec[i][column].size >= vec[row][column].size {
                    vec[row][column].increment_above();
                    break;
                } else {
                    vec[row][column].increment_above();
                }
            }
            // Check the trees below
            for i in row + 1..vec.len() {
                if vec[i][column].size >= vec[row][column].size {
                    vec[row][column].increment_below();
                    break;
                } else {
                    vec[row][column].increment_below();
                }
            }
            //tree.set_scenic_score();
        }
    }
    vec
}
