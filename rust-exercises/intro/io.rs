use std::fs;

fn main() {
    let rc = fs::write("file.txt", "Hello World\n")
    .expect("Failed to write to file");
}