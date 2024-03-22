use std::{fs::File, io::Write, path::Path};

use build_directory::build_directory_md;
fn main() -> Result<(), std::io::Error> {
    let mut file = File::create("DIRECTORY.md").unwrap(); // unwrap for panic

    match build_directory_md(Path::new(".")) {
        Ok(buf) => {
            file.write_all("# List of all files\n".as_bytes())?;
            file.write_all(buf.as_bytes())?;
        }
        Err(err) => {
            panic!("Error while creating string: {err}");
        }
    }
    Ok(())
}
