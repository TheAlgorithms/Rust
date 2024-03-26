use std::{
    fmt::Display,
    path::{Path, PathBuf},
};
use syn::{Item, UseTree, Visibility::Public};

pub fn main() {
    use std::{
        fs::{read_to_string, File},
        io::Write,
    };

    // Write exports to mod files.
    let mut exports_dir: ExportsDir = ExportsDir::new(Path::new("src"));
    exports_dir.write_to_mod_file();

    // Write the dir and file tree to the DIRECTORY.md file.
    let new_contents: String = "# List of all files\n\n##".to_string()
        + &UsageTree::new(Path::new("src").to_path_buf()).to_string()[1..];
    let current_contents: String =
        read_to_string("DIRECTORY.md").expect("Could not read DIRECTORY.md file");
    if current_contents != new_contents {
        File::create("DIRECTORY.md")
            .expect("Could not create DIRECTORY.md file")
            .write_all(new_contents.as_bytes())
            .expect("Could not write to DIRECTORY.md file");
    }
}

struct ExportsDir {
    pub path: PathBuf,
    pub name: String,
    pub files: Vec<ExportsFile>,
    pub sub_dirs: Vec<ExportsDir>,
}
impl ExportsDir {
    /// Collect a list of exports in a given directory. Does not recurse.
    fn new(directory: &Path) -> ExportsDir {
        use std::fs::read_dir;

        let exceptions: [&str; 3] = ["main.rs", "lib.rs", "mod.rs"];

        // Create initial instance.
        let path_buf: PathBuf = directory.to_path_buf();
        let mut exports: ExportsDir = ExportsDir {
            path: path_buf.clone(),
            name: path_buf
                .file_stem()
                .unwrap_or_else(|| {
                    panic!("Could not get filename of path '{}'", path_buf.display())
                })
                .to_str()
                .unwrap()
                .to_string(),
            files: Vec::new(),
            sub_dirs: Vec::new(),
        };

        // Populate instance with files and sub-dirs.
        let dir_entries = read_dir(directory)
            .unwrap_or_else(|_| panic!("Could not read dir {}", directory.display()))
            .flatten();
        for dir_entry in dir_entries {
            let dir_entry: PathBuf = dir_entry.path();
            if dir_entry.is_dir() {
                exports.sub_dirs.push(ExportsDir::new(&dir_entry));
            } else {
                let is_exception = exceptions
                    .iter()
                    .filter(|exception| dir_entry.ends_with(exception))
                    .count()
                    > 0;
                if !is_exception && dir_entry.extension().unwrap() == "rs" {
                    exports.files.push(ExportsFile::new(dir_entry));
                }
            }
        }

        // Return instance.
        exports
    }

    /// Remove some specific exports from all exports in the directory.
    pub fn remove_exports(&mut self, exclusions: &Vec<String>) {
        for sub_dir in &mut self.sub_dirs {
            sub_dir.remove_exports(exclusions);
        }
        for file in &mut self.files {
            file.remove_exports(exclusions);
        }
    }

    /// Get a flat list of all exports' names.
    pub fn flat_export_names(&self) -> Vec<&String> {
        let mut export_names = Vec::new();
        export_names.extend_from_slice(
            &self
                .files
                .iter()
                .flat_map(|file| &file.exports)
                .collect::<Vec<&String>>()[..],
        );
        export_names.extend_from_slice(
            &self
                .sub_dirs
                .iter()
                .flat_map(|dir| dir.flat_export_names())
                .collect::<Vec<&String>>()[..],
        );
        export_names
    }

    /// Prefix all duplicate exports with the name of their file.
    pub fn prefix_duplicates(&mut self) {
        for sub_dir in &mut self.sub_dirs {
            sub_dir.prefix_duplicates();
        }
        let exports_names: Vec<&String> = self.flat_export_names();
        let duplicate_names: Vec<String> = exports_names
            .iter()
            .enumerate()
            .filter(|(index, name)| {
                exports_names
                    .iter()
                    .position(|compare_name| &compare_name == name)
                    .unwrap()
                    != *index
            })
            .map(|(_, name)| name.to_string())
            .collect();
        for file in &mut self.files {
            file.prefix_exports(&duplicate_names);
        }
    }

    /// Remove all files that do not have any exports.
    pub fn remove_files_without_exports(&mut self) {
        for sub_dir in &mut self.sub_dirs {
            sub_dir.remove_files_without_exports();
        }
        let indexes_to_remove: Vec<usize> = self
            .files
            .iter()
            .enumerate()
            .filter(|(_, file)| file.exports.is_empty())
            .map(|(index, _)| index)
            .rev()
            .collect();
        for index in indexes_to_remove {
            self.files.remove(index);
        }
    }

    /// Write the exports to the mod file.
    pub fn write_to_mod_file(&mut self) {
        use regex::{Captures, Regex};
        use std::fs::{read_to_string, File};
        use std::io::Write;

        let start_tag_regex: Regex = Regex::new(
            r"\/\*\s+auto-exports\s+start\s+(exclusions\=\[(?<exclusions>.+)?\]\s+)?\*\/",
        )
        .unwrap();
        let end_tag_regex: Regex = Regex::new(r"\/\*\s+auto-exports\s+end\s+\*\/").unwrap();

        // Find the output path.
        let mut output_path: Option<PathBuf> = None;
        for addition in &["lib.rs", "mod.rs"] {
            let path = self.path.join(addition);
            if path.exists() {
                output_path = Some(path);
                break;
            }
        }
        if output_path.is_none() {
            panic!(
                "Could not find lib.rs or mod.rs file in dir '{}'",
                self.path.display()
            );
        }
        let output_path: PathBuf = output_path.unwrap();

        // Validate mod file exists.
        if !output_path.exists() {
            panic!("Mod file '{}' does not exist.", output_path.display());
        }
        if output_path.to_path_buf().is_dir() {
            panic!(
                "Cannot pass dir '{}' as mod file.",
                output_path.to_path_buf().display()
            );
        }
        let current_mod_contents: String = read_to_string(&output_path).unwrap_or_default();

        // Find auto-export start tag.
        let start_captures: Vec<Captures<'_>> = start_tag_regex
            .captures_iter(&current_mod_contents)
            .collect();
        if start_captures.is_empty() {
            return;
        }
        if start_captures.len() > 1 {
            panic!(
                "file '{}' has multiple auto-export start tags, which is not supported.",
                output_path.display()
            );
        }
        let start_capture = &start_captures[0];
        let start_capture_position: usize = start_capture.get(0).unwrap().end();
        let start_tag: &str = start_capture.get(0).unwrap().as_str();
        let exports_prefix: &str = &current_mod_contents[..start_capture.get(0).unwrap().start()];
        let export_exclusions: Vec<String> = start_capture
            .name("exclusions")
            .map(|capture_match| capture_match.as_str())
            .unwrap_or_default()
            .split(',')
            .map(|exclusion| exclusion.trim().to_string())
            .collect();

        // Find auto-export end-tag.
        let end_captures: Vec<Captures<'_>> = end_tag_regex
            .captures_iter(&current_mod_contents[start_capture_position..])
            .collect();
        if end_captures.is_empty() {
            panic!("Could not find auto-export end tag in file '{}', please add \"/* auto-exports end */\" somewhere.", output_path.display());
        }
        if end_captures.len() > 1 {
            panic!(
                "file '{}' has multiple auto-export end tags, which is not supported.",
                output_path.display()
            );
        }
        let end_capture = &end_captures[0];
        let end_capture_position: usize =
            start_capture_position + end_capture.get(0).unwrap().end();
        let end_tag: &str = end_capture.get(0).unwrap().as_str();
        let exports_suffix: &str = if end_capture_position < current_mod_contents.len() {
            &current_mod_contents[end_capture_position..]
        } else {
            ""
        };

        // Fix exclusions and duplicates.
        self.remove_exports(&export_exclusions);
        self.prefix_duplicates();
        self.remove_files_without_exports();

        // Parse exports into string.
        let new_mod_contents: String = format!(
            "{}{}\n{}\n{}{}",
            if exports_prefix.trim().is_empty() {
                ""
            } else {
                exports_prefix
            },
            start_tag,
            format!(
                "{}\n{}\n{}",
                self.sub_dirs
                    .iter()
                    .map(|dir| format!("pub mod {};", dir.name))
                    .collect::<Vec<String>>()
                    .join("\n"),
                self.files
                    .iter()
                    .map(|identity| identity.mod_to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
                self.files
                    .iter()
                    .map(|identity| identity.exports_to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
            .replace("\n\n", "\n")
            .trim(),
            end_tag,
            if exports_suffix.trim().is_empty() {
                ""
            } else {
                exports_suffix
            }
        );

        // If the new and current contents differ, write to file.
        if new_mod_contents != current_mod_contents {
            let mut mod_file = File::create(&output_path)
                .unwrap_or_else(|_| panic!("Could not access file '{}'", output_path.display()));
            mod_file
                .write_all(new_mod_contents.as_bytes())
                .unwrap_or_else(|_| panic!("Could not write to file '{}'.", output_path.display()));
        }

        // Recurse into sub-dirs.
        for dir in &mut self.sub_dirs {
            dir.write_to_mod_file();
        }
    }
}

struct ExportsFile {
    pub name: String,
    pub exports: Vec<String>,
}
impl ExportsFile {
    /// Create a new instance. Reads the file and collects exports automatically.
    pub fn new(path_buf: PathBuf) -> ExportsFile {
        use std::fs::read_to_string;

        // Create the intial instance.
        let mut exports_file = ExportsFile {
            name: path_buf
                .file_stem()
                .unwrap_or_else(|| {
                    panic!("Could not get filename of path '{}'", path_buf.display())
                })
                .to_str()
                .unwrap()
                .to_string(),
            exports: Vec::new(),
        };

        // Find all public exports in the file.
        let file_contents: String = read_to_string(&path_buf)
            .unwrap_or_else(|_| panic!("Could not read file '{}'", path_buf.display()));
        let syntax_tree = syn::parse_file(&file_contents)
            .unwrap_or_else(|_| panic!("Could not parse file '{}'", path_buf.display()));
        for item in syntax_tree.items {
            let identity: Option<String> = match item {
                Item::Fn(item) if matches!(item.vis, Public(_)) => Some(item.sig.ident.to_string()),
                Item::Struct(item) if matches!(item.vis, Public(_)) => Some(item.ident.to_string()),
                Item::Trait(item) if matches!(item.vis, Public(_)) => Some(item.ident.to_string()),
                Item::Enum(item) if matches!(item.vis, Public(_)) => Some(item.ident.to_string()),
                _ => None,
            };
            if let Some(identity) = identity {
                exports_file.exports.push(identity);
            }
        }

        // Return the exports file.
        exports_file
    }

    /// Remove some specific exports from the file.
    pub fn remove_exports(&mut self, removals: &[String]) {
        let indexes_to_remove: Vec<usize> = self
            .exports
            .iter()
            .enumerate()
            .filter(|(_, name)| removals.contains(name))
            .map(|(index, _)| index)
            .rev()
            .collect::<Vec<usize>>();
        for index in indexes_to_remove {
            self.exports.remove(index);
        }
    }

    /// Prefix the given exports with the name of the file.
    fn prefix_exports(&mut self, prefixed_exports: &[String]) {
        for export in &mut self.exports {
            if prefixed_exports.contains(&*export) {
                *export = format!("{} as {}_{}", export, self.name, export);
            }
        }
    }

    /// Create a string that exports the mod for the mod file.
    fn mod_to_string(&self) -> String {
        format!("mod {};", self.name)
    }

    /// Create a string that uses all exports for the mod file.
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

struct UsageTree {
    path: PathBuf,
    name: String,
    items: Vec<String>,
    sub_trees: Vec<UsageTree>,
}
impl UsageTree {
    /// Create a new exports tree given the starting directory.
    pub fn new(dir: PathBuf) -> UsageTree {
        use std::fs::read_to_string;

        if !dir.exists() || !dir.is_dir() {
            panic!(
                "Could not create UsageTree for directory '{}' as it does not exist.",
                dir.display()
            );
        }

        // Try for lib and mod file.
        let mut usages: Vec<String> = Vec::new();
        let mut sub_dirs: Vec<PathBuf> = Vec::new();
        for file_name in ["lib.rs", "mod.rs"] {
            let mod_file: PathBuf = dir.join(file_name);
            if mod_file.exists() {
                // Parse contents.
                let file_contents: String = read_to_string(&mod_file)
                    .unwrap_or_else(|_| panic!("Could not read file '{}'", mod_file.display()));
                let syntax_tree = syn::parse_file(&file_contents)
                    .unwrap_or_else(|_| panic!("Could not parse file '{}'", mod_file.display()));
                for item in syntax_tree.items {
                    match &item {
                        // 'pub use' suggests item in dir.
                        Item::Use(item) if matches!(item.vis, Public(_)) => {
                            match &item.tree {
                                UseTree::Path(path) => usages.push(path.ident.to_string()),
                                UseTree::Name(name) => usages.push(name.ident.to_string()),
                                UseTree::Rename(rename) => usages.push(rename.rename.to_string()),
                                _ => {}
                            };
                        }

                        // 'pub mod' suggests sub-dir.
                        Item::Mod(item) if matches!(item.vis, Public(_)) => {
                            sub_dirs.push(dir.join(item.ident.to_string()));
                        }

                        _ => {}
                    }
                }
            }
        }
        usages.sort();
        sub_dirs.sort();

        // Return usagetree.
        UsageTree {
            path: dir.clone(),
            name: dir
                .file_stem()
                .unwrap_or_else(|| panic!("Could not get filename of path '{}'", dir.display()))
                .to_str()
                .unwrap()
                .to_string(),
            items: usages,
            sub_trees: sub_dirs
                .iter()
                .map(|dir| UsageTree::new(dir.clone()))
                .collect::<Vec<UsageTree>>(),
        }
    }

    /// Format a file or dir name.
    pub fn pretty_name(name: &str) -> String {
        name.split('_')
            .map(|word| word[0..1].to_uppercase() + &word[1..])
            .collect::<Vec<String>>()
            .join(" ")
    }
}
impl Display for UsageTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spacing = "  ";
        let item_path: String = format!(
            "https://github.com/TheAlgorithms/Rust/blob/master/{}",
            self.path.display().to_string().replace('\\', "/")
        );

        // Parse files and dirs separately, then sort by name.
        let mut entries: Vec<(&str, String)> = Vec::new();
        for sub_tree in &self.sub_trees {
            entries.push((
                &sub_tree.name,
                sub_tree
                    .to_string()
                    .split('\n')
                    .map(|line| format!("{spacing}{line}"))
                    .collect::<Vec<String>>()
                    .join("\n"),
            ));
        }
        for item in &self.items {
            entries.push((
                item,
                format!(
                    "{spacing}* [{}]({}/{}.rs)",
                    Self::pretty_name(item),
                    item_path,
                    item
                ),
            ));
        }
        entries.sort_by(|a, b| a.0.cmp(b.0));

        write!(
            f,
            "* {}\n{}",
            Self::pretty_name(&self.name),
            entries
                .iter()
                .map(|entry| entry.1.clone())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
