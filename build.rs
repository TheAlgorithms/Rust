use std::{
    fs::{read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

static AUTO_IMPORT_START_STR: &str = "/* start auto-imports */";
static AUTO_IMPORT_END_STR: &str = "/* end auto-imports */";

pub fn main() {
    auto_export_mod(Path::new("src/lib.rs"));
}

/// Generates the 'mod' and 'use' expressions for a lib.rs or mod.rs file to export everything available.
fn auto_export_mod(mod_file_path: &Path) {
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

    // Parse current contents.
    let current_mod_contents: String = read_to_string(mod_file_path).unwrap_or_default();
    let imports_prefix: &str = current_mod_contents
        .split(AUTO_IMPORT_START_STR)
        .next()
        .unwrap();
    let imports_suffix: &str = if current_mod_contents.contains(AUTO_IMPORT_END_STR) {
        current_mod_contents
            .split(AUTO_IMPORT_END_STR)
            .collect::<Vec<&str>>()[1]
    } else {
        ""
    };

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
        for pub_identity in &mut export_files {
            let file_name: &str = &pub_identity.name;
            for export in &mut pub_identity.exports {
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
        AUTO_IMPORT_START_STR,
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
        AUTO_IMPORT_END_STR,
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
                "pub use {}::{} {} {};",
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
