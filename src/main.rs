use std::io::{self, Write};

fn check_if_exists(file_path: &str) -> bool {
    std::path::Path::new(file_path).exists()
}

fn main() {
    print!("Where is the file you want to compress?");
    io::stdout().flush().unwrap();
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).unwrap();

    file_path = file_path.trim().to_string();

    if !check_if_exists(&file_path) {
        println!("File does not exist");
        return;
    }

    println!("File exists");
}
