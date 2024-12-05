use flate2::{write::GzEncoder, Compression};
use std::{
    fs::File,
    io::{Cursor, Read, Result, Write},
    path::Path,
};
use tar::Builder;

pub fn compress_file(file_path: &str) -> Result<()> {
    let mut file = File::open(&file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());

    encoder.write_all(&buffer)?;
    let compressed_data = encoder.finish()?;
    let compressed_file_path = format!("{}.gzip", file_path);
    let mut compressed_file = File::create(&compressed_file_path)?;
    compressed_file.write_all(&compressed_data)?;
    Ok(())
}

pub fn compress_folder_as_gzip(folder_path: &str, output_file: &str) -> Result<()> {
    let tar_file = Vec::new();
    let tar_writer = Builder::new(Cursor::new(tar_file));

    let tar_writer = add_folder_to_archive(folder_path, tar_writer)?;

    let tar_data = tar_writer
        .into_inner()
        .unwrap_or_else(|err| panic!("Error in adding folder to archive: {}", err))
        .into_inner();

    let compressed_file = File::create(output_file)?;
    let mut encoder = GzEncoder::new(compressed_file, Compression::best());
    encoder.write_all(&tar_data)?;
    encoder.finish()?;

    Ok(())
}
fn add_folder_to_archive<W: Write>(
    folder_path: &str,
    mut builder: Builder<W>,
) -> Result<Builder<W>> {
    let folder = Path::new(folder_path);

    if folder.is_dir() {
        for entry in folder.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                builder = add_folder_to_archive(
                    path.to_str()
                        .unwrap_or_else(|| panic!("Error in adding folder to archive")),
                    builder,
                )?;
            } else if path.is_file() {
                let mut file = File::open(&path)?;
                let relative_path = path.strip_prefix(folder_path).unwrap();
                builder.append_file(
                    relative_path
                        .to_str()
                        .unwrap_or_else(|| panic!("Error stripping prefix")),
                    &mut file,
                )?;
            }
        }
    }
    Ok(builder)
}
