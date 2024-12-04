use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use std::path::Path;

fn check_if_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

fn compress_file(file_path: &str) {
    let mut file = File::open(&file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());

    encoder.write_all(&buffer).unwrap();
    let compressed_data = encoder.finish().unwrap();
    let compressed_file_path = format!("{}.gzip", file_path);
    let mut compressed_file = File::create(&compressed_file_path).unwrap();
    compressed_file.write_all(&compressed_data).unwrap();
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

    compress_file(&file_path);

    println!("File compressed successfully");
}
