use dirs::data_dir;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};
/// Constructs the full path to the settings file for a given key.
/// Files are stored in a platform-specific application data directory.
fn get_settings_file_path(key: &str) -> io::Result<PathBuf> {
    let mut path = data_dir()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine user data directory",
            )
        })?;
    path.push("AnyTasks");
    fs::create_dir_all(&path)?;
    path.push(format!("{}.json", key));
    Ok(path)
}
/// Saves data to a file in JSON format.
///
/// `key`: A unique identifier for the data, used as part of the filename.
/// `data`: The data structure to save, which must implement `Serialize`.
pub fn save_data<T: Serialize>(key: &str, data: &T) -> io::Result<()> {
    let file_path = get_settings_file_path(key)?;
    let serialized = serde_json::to_string_pretty(data)?;
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
    let contents = fs::read_to_string(&file_path)?;
    let data = serde_json::from_str(&contents)?;
    Ok(data)
}
