use std::{fs::File, io::Read};
fn main() {
    let contents = read_input();
    find_unique_slice(contents.clone(), 4);
    find_unique_slice(contents.clone(), 14);
}
fn find_unique_slice(contents: Vec<char>, window_size: usize) {
    let now = std::time::Instant::now();
    let iter = contents.windows(window_size);
    for i in iter.enumerate() {
        let (window, index) = (i.1, i.0);
        if !(1..window.len()).any(|i| window[i..].contains(&window[i - 1])) {
            println!(
                "Window: {:?} index: {index} offset({window_size}): {}",
                window,
                index + window_size
            );
            println!("Execution time: {:?}", now.elapsed());
            break;
        }
    }
}
fn read_input() -> Vec<char> {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.chars().collect()
}
