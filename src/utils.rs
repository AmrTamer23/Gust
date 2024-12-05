use std::path::Path;

pub fn check_if_exists(file_path: &str) -> &str {
    let path = Path::new(file_path);
    let is_existing = path.exists();

    if !is_existing {
        return "false";
    } else {
        if path.is_dir() {
            return "folder";
        } else {
            return "file";
        }
    }
}
