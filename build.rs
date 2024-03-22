use std::{ fs::{read_to_string, File}, io::Write, path::{Path, PathBuf} };


static AUTO_IMPORT_START_STR:&str = "/* start auto-imports */";
static AUTO_IMPORT_END_STR:&str = "/* end auto-imports */";


pub fn main() {
    auto_export_mod(&Path::new("src/lib.rs"));
}



/// Generates the 'mod' and 'use' expressions for a lib.rs or mod.rs file to export everything available.
pub fn auto_export_mod(mod_file_path: &Path) {

    // Validate mod file exists.
    if !mod_file_path.exists() {
        panic!("Mod file '{}' does not exist.", mod_file_path.display());
    }
    if mod_file_path.to_path_buf().is_dir() {
        panic!("Cannot pass dir '{}' as mod file.", mod_file_path.to_path_buf().display());
    }
    
    // Get exports.
    let mod_dir: &Path = mod_file_path.parent().expect(&format!("Could not get directory of file '{}'", mod_file_path.display()));
    let exports: Vec<Export> = collect_exports(mod_dir);

    // Recurse into directories.
    for dir in exports.iter().filter(|entry| entry.is_dir) {
        auto_export_mod(mod_dir.join(&dir.name).join("mod.rs").as_path());
    }

    // Parse current contents.
    let current_mod_contents:String = read_to_string(mod_file_path).unwrap_or_default();
    let imports_prefix:&str = current_mod_contents.split(AUTO_IMPORT_START_STR).next().unwrap();
    let imports_suffix:&str = if current_mod_contents.contains(AUTO_IMPORT_END_STR) { current_mod_contents.split(AUTO_IMPORT_END_STR).collect::<Vec<&str>>()[1] } else { "" };

    // Parse imports into string.
    let new_mod_contents:String = format!(
        "{}{}\n{}\n{}{}",
        if imports_prefix.trim().len() > 0 { imports_prefix } else { "" },
        AUTO_IMPORT_START_STR,
        format!(
            "{}\n{}\n{}",
            exports.iter().filter(|e| e.is_dir).map(|e| format!("pub mod {};", e.name)).collect::<Vec<String>>().join("\n"),
            exports.iter().filter(|e| !e.is_dir).map(|e| format!("mod {};", e.name)).collect::<Vec<String>>().join("\n"),
            exports.iter().filter(|e| !e.is_dir).map(|e| format!("pub use {}::*;", e.name)).collect::<Vec<String>>().join("\n"),
        ).replace("\n\n", "\n").trim(),
        AUTO_IMPORT_END_STR,
        if imports_suffix.trim().len() > 0 { imports_suffix } else { "" }
    );
    
    // If the new and current contents differ, write to file.
    if new_mod_contents != current_mod_contents {
        let mut mod_file = File::create(mod_file_path).expect(&format!("Could not access file '{}'", mod_file_path.display()));
        mod_file.write_all(&new_mod_contents.as_bytes()[..]).expect(&format!("Could not write to file '{}'.", mod_file_path.display()));
    }
}




static EXPORT_COLLECTION_EXCEPTIONS:[&str; 3] = ["main.rs", "lib.rs", "mod.rs"];

/// Collect a list of exports in a given directory. Does not recurse.
fn collect_exports(directory: &Path) -> Vec<Export> {
    use std::fs::read_dir;
    
    let mut exports: Vec<Export> = Vec::new();
    let dir_entries = read_dir(directory).expect(&format!("Could not read dir {}", directory.display())).flatten();
    for dir_entry in dir_entries {
        let dir_entry: PathBuf = dir_entry.path();
        if (dir_entry.is_dir() || dir_entry.extension().unwrap() == "rs") && EXPORT_COLLECTION_EXCEPTIONS.iter().filter(|exception| dir_entry.ends_with(exception)).count() == 0 {
            exports.push(Export::new(&dir_entry));
        }
    }
    exports
}



pub struct Export {
    pub name: String,
    pub is_dir: bool
}
impl Export {
    pub fn new(path_buf: &PathBuf) -> Export {
        Export {
            name: path_buf.file_stem().expect(&format!("Could not get filename of path '{}'", path_buf.display())).to_str().unwrap().to_string(),
            is_dir: path_buf.is_dir()
        }
    }
}