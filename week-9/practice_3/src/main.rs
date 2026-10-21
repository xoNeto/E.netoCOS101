use std::fs;
fn main() {
    fs::remove_file("../practice_1/data.txt").expect("could not remove file");
    println!("file is removed");
}