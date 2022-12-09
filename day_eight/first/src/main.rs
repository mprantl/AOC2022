use std::{fs::File, io::Read};
#[derive(Debug, Clone, Copy)]
struct Tree {
    size: u32,
    is_visible: bool,
}
impl Tree {
    fn new(size: u32) -> Tree {
        Tree {
            size,
            is_visible: false,
        }
    }
    fn set_visible(&mut self) {
        self.is_visible = true;
    }
}
fn main() {
    let mut vec = read_file();
    vec = visible_tree_amount(vec);
    let mut counter = 0;
    for row in vec.iter() {
        for tree in row.iter() {
            if tree.is_visible {
                counter += 1;
            }
        }
    }
    println!("\n{} Trees are visible", counter);
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
    let mut current_max: u32 = 0;
    //Rows
    for row in vec.iter_mut() {
        for tree in row.iter_mut() {
            if tree.size > current_max {
                current_max = tree.size;
                tree.set_visible();
            }
        }
        current_max = 0;
        for tree in row.iter_mut().rev() {
            if tree.size > current_max {
                current_max = tree.size;
                tree.set_visible();
            }
        }
        current_max = 0;
    }
    //Columns
    for i in 0..vec[0].len() {
        for row in vec.iter_mut() {
            let tree = &mut row[i];
            if tree.size > current_max {
                current_max = tree.size;
                tree.set_visible();
            }
        }
        current_max = 0;
        for row in vec.iter_mut().rev() {
            let tree = &mut row[i];
            if tree.size > current_max {
                current_max = tree.size;
                tree.set_visible();
            }
        }
        current_max = 0;
    }
    let vec_len = vec.len();
    for (i, row) in vec.iter_mut().enumerate() {
        for (j, tree) in row.iter_mut().enumerate() {
            if (i == 0 || j == 0) || (i == vec_len - 1 || j == vec_len - 1) {
                tree.set_visible();
            }
        }
    }
    vec
}
