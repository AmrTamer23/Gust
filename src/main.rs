pub mod compress;
pub mod utils;

use std::io;

use compress::{compress_file, compress_folder_as_gzip};

use utils::check_if_exists;

fn main() {
    println!("Enter the folder you want to compress:");
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    let path_type = check_if_exists(path);

    if path_type == "false" {
        println!("Folder does not exist.");
        return;
    } else if path_type == "folder" {
        let output_folder = format!("{}.tar.gz", path);
        match compress_folder_as_gzip(path, &output_folder) {
            Ok(_) => println!("Folder compressed successfully as {}.", output_folder),
            Err(err) => println!("Error compressing folder: {}", err),
        }
    } else if path_type == "file" {
        let output_file = format!("{}.gzip", path);
        match compress_file(path) {
            Ok(_) => println!("File compressed successfully as {}", output_file),
            Err(err) => println!("Error compressing the file: {}", err),
        }
    }
}
