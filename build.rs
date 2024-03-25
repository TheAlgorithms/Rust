use std::{
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

pub fn main() {
    auto_export_mod(Path::new("src/lib.rs"));
}

/// Generates the 'mod' and 'use' expressions for a lib.rs or mod.rs file to export everything available.
fn auto_export_mod(mod_file_path: &Path) {
    use regex::{Captures, Regex};

    let start_tag_regex: Regex =
        Regex::new(r"\/\*\s+auto-imports\s+start\s+(exclusions\=\[(?<exclusions>.+)?\]\s+)?\*\/")
            .unwrap();
    let end_tag_regex: Regex = Regex::new(r"\/\*\s+auto-imports\s+end\s+\*\/").unwrap();

    // Validate mod file exists.
    if !mod_file_path.exists() {
        panic!("Mod file '{}' does not exist.", mod_file_path.display());
    }
    if mod_file_path.to_path_buf().is_dir() {
        panic!(
            "Cannot pass dir '{}' as mod file.",
            mod_file_path.to_path_buf().display()
        );
    }
    let current_mod_contents: String = read_to_string(mod_file_path).unwrap_or_default();

    // Find auto-import start tag.
    let start_captures: Vec<Captures<'_>> = start_tag_regex
        .captures_iter(&current_mod_contents)
        .collect();
    if start_captures.is_empty() {
        return;
    }
    if start_captures.len() > 1 {
        panic!(
            "file '{}' has multiple auto-export start tags, which is not supported.",
            mod_file_path.display()
        );
    }
    let start_capture = &start_captures[0];
    let start_capture_position: usize = start_capture.get(0).unwrap().end();
    let start_tag: &str = start_capture.get(0).unwrap().as_str();
    let imports_prefix: &str = &current_mod_contents[..start_capture.get(0).unwrap().start()];
    let imports_exclusions: Vec<String> = start_capture
        .name("exclusions")
        .map(|capture_match| capture_match.as_str())
        .unwrap_or_default()
        .split(',')
        .map(|exclusion| exclusion.trim().to_string())
        .collect();

    // Find auto-import end-tag.
    let end_captures: Vec<Captures<'_>> = end_tag_regex
        .captures_iter(&current_mod_contents[start_capture_position..])
        .collect();
    if end_captures.is_empty() {
        panic!("Could not find auto-import end tag in file '{}', please add \"/* auto-imports end */\" somewhere.", mod_file_path.display());
    }
    if end_captures.len() > 1 {
        panic!(
            "file '{}' has multiple auto-export end tags, which is not supported.",
            mod_file_path.display()
        );
    }
    let end_capture = &end_captures[0];
    let end_capture_position: usize = start_capture_position + end_capture.get(0).unwrap().end();
    let end_tag: &str = end_capture.get(0).unwrap().as_str();
    let imports_suffix: &str = if end_capture_position < current_mod_contents.len() {
        &current_mod_contents[end_capture_position..]
    } else {
        ""
    };

    // Get exports.
    let mod_dir: &Path = mod_file_path.parent().unwrap_or_else(|| {
        panic!(
            "Could not get directory of file '{}'",
            mod_file_path.display()
        )
    });
    let exports: Vec<Export> = collect_exports(mod_dir);

    // Recurse into directories.
    for dir in exports.iter().filter(|entry| entry.is_dir) {
        auto_export_mod(mod_dir.join(&dir.name).join("mod.rs").as_path());
    }

    // Find the exported pub things for all files.
    let mut export_files: Vec<ExportsFile> = Vec::new();
    for export in exports.iter().filter(|export| !export.is_dir) {
        let file = mod_dir.join(export.name.clone() + ".rs");
        let contents = read_to_string(&file)
            .unwrap_or_else(|_| panic!("Could not read file '{}'", file.display()));
        let file_exports: Vec<String> = find_exports(&contents);
        if !file_exports.is_empty() {
            export_files.push(ExportsFile::new(export.name.clone(), file_exports));
        }
    }

    // Remove excluded pub identities.
    for export_file in &mut export_files {
        let exclusion_violations: Vec<usize> = export_file
            .exports
            .iter_mut()
            .enumerate()
            .filter(|(_, export)| imports_exclusions.contains(export))
            .rev()
            .map(|(index, _)| index)
            .collect();
        for index in exclusion_violations {
            export_file.exports.remove(index);
        }
    }

    // Prefix duplicates in pub identities.
    let flat_names: Vec<&String> = export_files
        .iter()
        .flat_map(|identity| &identity.exports)
        .collect::<Vec<&String>>();
    let duplicate_names: Vec<String> = flat_names
        .iter()
        .enumerate()
        .filter(|(index, name)| flat_names.iter().position(|e| &e == name).unwrap() != *index)
        .map(|(_, name)| name.to_string())
        .collect::<Vec<String>>();
    if !duplicate_names.is_empty() {
        for export_file in &mut export_files {
            let file_name: &str = &export_file.name;
            for export in &mut export_file.exports {
                if duplicate_names.contains(export) {
                    *export = format!("{export} as {file_name}_{export}");
                }
            }
        }
    }

    // Parse imports into string.
    let new_mod_contents: String = format!(
        "{}{}\n{}\n{}{}",
        if !imports_prefix.trim().is_empty() {
            imports_prefix
        } else {
            ""
        },
        start_tag,
        format!(
            "{}\n{}\n{}",
            exports
                .iter()
                .filter(|e| e.is_dir)
                .map(|e| format!("pub mod {};", e.name))
                .collect::<Vec<String>>()
                .join("\n"),
            export_files
                .iter()
                .map(|identity| identity.mod_to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            export_files
                .iter()
                .map(|identity| identity.exports_to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .replace("\n\n", "\n")
        .trim(),
        end_tag,
        if !imports_suffix.trim().is_empty() {
            imports_suffix
        } else {
            ""
        }
    );

    // If the new and current contents differ, write to file.
    if new_mod_contents != current_mod_contents {
        let mut mod_file = File::create(mod_file_path)
            .unwrap_or_else(|_| panic!("Could not access file '{}'", mod_file_path.display()));
        mod_file
            .write_all(new_mod_contents.as_bytes())
            .unwrap_or_else(|_| panic!("Could not write to file '{}'.", mod_file_path.display()));
    }
}

/// Returns a list of the names of all public methods in the root of the given rust code.
fn find_exports(code_contents: &str) -> Vec<String> {
    use syn::{Item, Visibility::Public};
    let mut method_names: Vec<String> = Vec::new();

    let syntax_tree = syn::parse_file(code_contents).expect("Failed to parse file");
    for item in syntax_tree.items {
        match item {
            Item::Fn(item) if matches!(item.vis, Public(_)) => {
                method_names.push(item.sig.ident.to_string());
            }
            Item::Struct(item) if matches!(item.vis, Public(_)) => {
                method_names.push(item.ident.to_string());
            }
            Item::Trait(item) if matches!(item.vis, Public(_)) => {
                method_names.push(item.ident.to_string());
            }
            Item::Enum(item) if matches!(item.vis, Public(_)) => {
                method_names.push(item.ident.to_string());
            }
            _ => {}
        }
    }

    method_names
}

static EXPORT_COLLECTION_EXCEPTIONS: [&str; 3] = ["main.rs", "lib.rs", "mod.rs"];

/// Collect a list of exports in a given directory. Does not recurse.
fn collect_exports(directory: &Path) -> Vec<Export> {
    use std::fs::read_dir;

    let mut exports: Vec<Export> = Vec::new();
    let dir_entries = read_dir(directory)
        .unwrap_or_else(|_| panic!("Could not read dir {}", directory.display()))
        .flatten();
    for dir_entry in dir_entries {
        let dir_entry: PathBuf = dir_entry.path();
        let is_exception = EXPORT_COLLECTION_EXCEPTIONS
            .iter()
            .filter(|exception| dir_entry.ends_with(exception))
            .count()
            > 0;
        if !is_exception && (dir_entry.is_dir() || dir_entry.extension().unwrap() == "rs") {
            exports.push(Export::new(dir_entry));
        }
    }
    exports
}

struct ExportsFile {
    pub name: String,
    pub exports: Vec<String>,
}
impl ExportsFile {
    pub fn new(name: String, exports: Vec<String>) -> ExportsFile {
        ExportsFile { name, exports }
    }

    fn mod_to_string(&self) -> String {
        format!("mod {};", self.name)
    }

    fn exports_to_string(&self) -> String {
        match self.exports.len() {
            0 => String::new(),
            1 => format!("pub use {}::{};", self.name, self.exports[0]),
            _ => format!(
                "pub use {}::{}{}{};",
                self.name,
                '{',
                self.exports.join(", "),
                '}'
            ),
        }
    }
}

struct Export {
    pub name: String,
    pub is_dir: bool,
}
impl Export {
    pub fn new(path_buf: PathBuf) -> Export {
        Export {
            name: path_buf
                .file_stem()
                .unwrap_or_else(|| {
                    panic!("Could not get filename of path '{}'", path_buf.display())
                })
                .to_str()
                .unwrap()
                .to_string(),
            is_dir: path_buf.is_dir(),
        }
    }
}
