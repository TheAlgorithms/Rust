use std::error::Error;
use std::fs;
use std::path::Path;

/// Lists All the basenames or say directory and file names
/// in a directory and returns a Vector of Strings.
///
/// # Arguments
///
/// * `directory` - A Path like object
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::file_system::list_basenames;
///
/// let directory_listing: Vec<String> = list_basenames(".").expect("Unable to list directory, an error occured");
/// ```
pub fn list_basenames<P: AsRef<Path>>(directory: P) -> Result<Vec<String>, Box<dyn Error>> {
    // Read the contents of the directory
    let directory = fs::read_dir(directory)?;

    // Create an empty vector to contain base_names
    let mut base_names: Vec<String> = vec![];

    // Consume directory
    for dir_entry in directory {
        // Get entry's base_name if exists.
        if let Some(file_name) = dir_entry?.path().file_name() {
            // Convert it to &str if possible
            if let Some(value) = file_name.to_str() {
                // Push the resulting value to Vector after converting to String
                base_names.push(String::from(value));
            }
        }
    }

    Ok(base_names)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_listing() {
        use crate::file_system::list_basenames;

        let expected_result: Vec<String> = vec![
            String::from("dummy_text.txt"),
            String::from("dummy_directory2"),
        ];
        let actual_result: Vec<String> = list_basenames("src/file_system/dummy_directory").unwrap();

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_listing_not_equal() {
        use crate::file_system::list_basenames;

        let unexpected_result: Vec<String> = vec![
            String::from("non_existant zhdjdbkadladkg.txt"),
            String::from("non_existant gibrihdjgagdag"),
        ];
        let actual_result: Vec<String> = list_basenames("src/file_system/dummy_directory").unwrap();

        assert_ne!(actual_result, unexpected_result);
    }
}
