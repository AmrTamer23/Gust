use std::path::Path;

pub fn check_if_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}
