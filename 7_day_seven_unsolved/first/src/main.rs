use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Clone, Default)]
struct DirNode {
    parent: Option<Box<DirNode>>,
    children: Option<Vec<DirNode>>,
    files: Option<Vec<FileNode>>,
    name: String,
    sum: u128,
}
//struct file node
#[derive(Debug, PartialEq, Clone, Default)]
struct FileNode {
    parent: Option<Box<DirNode>>,
    name: String,
    size: u128,
}
//impl for DirNode
impl DirNode {
    fn create(name: String, parent: Option<Box<DirNode>>) -> Self {
        DirNode {
            parent,
            children: None,
            files: None,
            name: name,
            sum: 0,
        }
    }
    fn root() -> Self {
        DirNode {
            parent: None,
            children: None,
            files: None,
            name: String::from("/"),
            sum: 0,
        }
    }
    fn add_dir(&mut self, name: String) {
        let new_dir = DirNode::create(name, Some(Box::new(self.clone())));
        self.children
            .as_mut()
            .unwrap_or(&mut Vec::new())
            //.unwrap_or_default()
            .push(new_dir);
    }

    fn add_file(&mut self, name: String, data: u128) {
        let new_file = FileNode::new(name, data, Some(Box::new(self.clone())));
        self.files.as_mut().unwrap().push(new_file);
    }
}
impl FileNode {
    fn new(name: String, data: u128, parent: Option<Box<DirNode>>) -> Self {
        FileNode {
            parent,
            name,
            size: data,
        }
    }
}
fn main() {
    let input = read_input();
    /*
    for line in input {
        let mut line_iter = line.split_whitespace();
        if line_iter.nth(1).unwrap() == "cd" {
            println!("cd");
        }
        //println!("{}", line);
    }*/
    build_dir_tree(input);
}
fn read_input() -> Vec<String> {
    let mut file = File::open("input_smoll.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let mut input: Vec<String> = Vec::new();
    for line in contents.lines() {
        input.push(line.to_string());
    }
    input
}
fn build_dir_tree(input: Vec<String>) {
    for line in input.iter() {
        let mut line_iter = line.split_whitespace();
        let mut current_dir: DirNode = DirNode::root(); // = root_dir.clone();
        match line_iter.next().expect("end of content") {
            "$" => {
                match line_iter.next().expect("end of content1") {
                    "cd" => {
                        match line_iter.next().expect("end of content 2") {
                            "/" => println!("/ root"), //current_dir = DirNode::root(), println!("/"), //     //current_dir = root_dir.clone(),
                            ".." => current_dir = *current_dir.parent.unwrap(), //println!(".. parent"),
                            //Some(x) => current_dir = current_dir.children.into_iter().collect(), //println!("change to dirname"),
                            _ => println!(
                                "error {:?} \t {:?} \t {:?}",
                                current_dir.parent,
                                current_dir.name,
                                current_dir
                                    .children
                                    .iter()
                                    .any(|x| x. == "ddpgzpc")
                            ),
                        };
                        //println!("{:?}", current_dir.name);
                    }
                    "ls" => println!("ls: \n {:?}", current_dir),
                    _ => println!("error"),
                }
            }
            "dir" => current_dir.add_dir(line_iter.next().unwrap().to_string()), /*println!(
            "addDir{:?} {:?}",
            line_iter.next().unwrap(),
            current_dir.name
            ), */
            _ => println!("addFile{:?}", line_iter.next().unwrap()),
            /*
            current_dir.add_file(
                line_iter.next().unwrap().to_string(),
                //line_iter.next().unwrap().to_string(),
                1,),*/
        }
    }
}
