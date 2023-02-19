use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    let file_content = fs::read_to_string(file_path).expect("File not found :(");

    println!(
        "Searching for {} in {} with texts: \n\n{}",
        query, file_path, file_content
    );
}
