use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

static URL_BASE: &str = "https://github.com/TheAlgorithms/Rust/blob/master";

fn good_filepaths(top_dir: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut good_fs = Vec::new();
    if top_dir.is_dir() {
        for entry in fs::read_dir(top_dir)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_name().to_str().unwrap().starts_with('.')
                || entry.file_name().to_str().unwrap().starts_with('_')
            {
                continue;
            }
            if path.is_dir() {
                let mut other = good_filepaths(&path)?;
                good_fs.append(&mut other);
            } else if entry.file_name().to_str().unwrap().ends_with(".rs")
                && entry.file_name().to_str().unwrap() != "mod.rs"
            {
                good_fs.push(
                    path.into_os_string()
                        .into_string()
                        .unwrap()
                        .split_at(2)
                        .1
                        .to_string(),
                );
            }
        }
    }
    good_fs.sort();
    Ok(good_fs)
}

fn md_prefix(indent_count: usize) -> String {
    if indent_count > 0 {
        format!("{}*", "  ".repeat(indent_count))
    } else {
        "\n##".to_string()
    }
}

fn print_path(old_path: String, new_path: String) -> (String, String) {
    let old_parts = old_path
        .split(std::path::MAIN_SEPARATOR)
        .collect::<Vec<&str>>();
    let mut result = String::new();
    for (count, new_part) in new_path.split(std::path::MAIN_SEPARATOR).enumerate() {
        if count + 1 > old_parts.len() || old_parts[count] != new_part {
            println!("{} {}", md_prefix(count), to_title(new_part));
            result.push_str(format!("{} {}\n", md_prefix(count), to_title(new_part)).as_str());
        }
    }
    (new_path, result)
}

pub fn build_directory_md(top_dir: &Path) -> Result<String, Box<dyn Error>> {
    let mut old_path = String::from("");
    let mut result = String::new();
    for filepath in good_filepaths(top_dir)? {
        let mut filepath = PathBuf::from(filepath);
        let filename = filepath.file_name().unwrap().to_owned();
        filepath.pop();
        let filepath = filepath.into_os_string().into_string().unwrap();
        if filepath != old_path {
            let path_res = print_path(old_path, filepath);
            old_path = path_res.0;
            result.push_str(path_res.1.as_str());
        }
        let url = format!("{}/{}", old_path, filename.to_string_lossy());
        let url = get_addr(&url);
        let indent = old_path.matches(std::path::MAIN_SEPARATOR).count() + 1;
        let filename = to_title(filename.to_str().unwrap().split('.').collect::<Vec<&str>>()[0]);
        println!("{} [{}]({})", md_prefix(indent), filename, url);
        result.push_str(format!("{} [{}]({})\n", md_prefix(indent), filename, url).as_str());
    }
    Ok(result)
}

fn to_title(name: &str) -> String {
    let mut change = true;
    name.chars()
        .map(move |letter| {
            if change && !letter.is_numeric() {
                change = false;
                letter.to_uppercase().next().unwrap()
            } else if letter == '_' {
                change = true;
                ' '
            } else {
                if letter.is_numeric() || !letter.is_alphanumeric() {
                    change = true;
                }
                letter
            }
        })
        .collect::<String>()
}

fn get_addr(addr: &str) -> String {
    if cfg!(windows) {
        format!("{}/{}", URL_BASE, switch_backslash(addr))
    } else {
        format!("{}/{}", URL_BASE, addr)
    }
}

// Function that changes '\' to '/' (for Windows builds only)
fn switch_backslash(addr: &str) -> String {
    addr.chars()
        .map(|mut symbol| {
            if symbol == '\\' {
                symbol = '/';
            }
            symbol
        })
        .collect::<String>()
}
