pub mod compress;
pub mod utils;

use compress::compress_file;
use std::io::{self, Write};
use utils::check_if_exists;

fn main() {
    println!("Where is the file you want to compress?");
    io::stdout().flush().unwrap();
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).unwrap();

    file_path = file_path.trim().to_string();

    if !check_if_exists(&file_path) {
        println!("File does not exist");
        return;
    }

    compress_file(&file_path);

    println!("File compressed successfully");
}
