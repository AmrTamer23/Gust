use std::{
    fs::File,
    io::{Read, Write},
};

use flate2::{write::GzEncoder, Compression};

pub fn compress_file(file_path: &str) {
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
