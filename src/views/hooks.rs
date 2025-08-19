use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};
use dirs::data_dir; // For cross-platform data directory discovery


/// Constructs the full path to the settings file for a given key.
/// Files are stored in a platform-specific application data directory.
fn get_settings_file_path(key: &str) -> io::Result<PathBuf> {
    // Get the user's data directory (e.g., ~/.config/app_name on Linux,
    // ~/Library/Application Support/app_name on macOS, %APPDATA%/app_name on Windows)
    let mut path = data_dir().ok_or_else(|| io::Error::new(
        io::ErrorKind::NotFound,
        "Could not determine user data directory",
    ))?;

    // IMPORTANT: Replace "your_app_name" with a unique name for your application
    path.push("AnyTasks");
    
    // Ensure the application-specific directory exists
    fs::create_dir_all(&path)?;

    // Add the filename, based on the key
    path.push(format!("{}.json", key));
    Ok(path)
}

/// Saves data to a file in JSON format.
///
/// `key`: A unique identifier for the data, used as part of the filename.
/// `data`: The data structure to save, which must implement `Serialize`.
pub fn save_data<T: Serialize>(key: &str, data: &T) -> io::Result<()> {
    let file_path = get_settings_file_path(key)?;
    
    // Serialize the data to a JSON string
    let serialized = serde_json::to_string_pretty(data)?;

    // Write the JSON string to the file
    let mut file = File::create(&file_path)?;
    file.write_all(serialized.as_bytes())?;
    
    Ok(())
}

/// Retrieves data from a file, expecting it in JSON format.
///
/// `key`: The unique identifier for the data.
/// Returns `Ok(T)` if successful, or an `io::Error` if the file is not found,
/// cannot be read, or deserialization fails.
pub fn get_data<T: DeserializeOwned + Default>(key: &str) -> io::Result<T> {
    let file_path = get_settings_file_path(key)?;

    // Read the file content as a string
    let contents = fs::read_to_string(&file_path)?;
    
    // Deserialize the JSON string into the specified data type
    let data = serde_json::from_str(&contents)?;
    Ok(data)
}
