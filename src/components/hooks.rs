use dirs::data_dir;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};
// Required for Android-specific path retrieval
// #[cfg(target_os = "android")]
// use anyhow::anyhow;
// #[cfg(target_os = "android")]
// use jni::{
//     objects::{JObject, JString},
//     JNIEnv,
// };

// #[cfg(target_os = "android")]
// // use once_cell::sync::LazyLock;
// use std::sync::LazyLock;
// #[cfg(target_os = "android")]
// use dioxus::mobile::wry::prelude::dispatch;


// #[derive(Clone)]
// pub struct AndroidStorage;

// impl StorageBacking for AndroidStorage {
//     type Key = String;

//     fn get<T: DeserializeOwned + Clone + 'static>(key: &Self::Key) -> Option<T> {
//         let path = FILE_DIR.join(format!("{}.json", key));
//         let mut file = File::open(path).ok()?;
//         let mut contents = String::new();
//         file.read_to_string(&mut contents).ok()?;
//         serde_json::from_str(&contents).ok()
//     }

//     fn set<T: Serialize + Send + Sync + Clone + 'static>(key: Self::Key, value: &T) {
//         let path = FILE_DIR.join(format!("{}.json", key));
//         if let Ok(json) = serde_json::to_string_pretty(value) {
//             if let Some(parent) = path.parent() {
//                 let _ = fs::create_dir_all(parent);
//             }
//             if let Ok(mut file) = File::create(path) {
//                 let _ = file.write_all(json.as_bytes());
//             }
//         }
//     }
// }
// #[cfg(target_os = "android")]
// static FILE_DIR: LazyLock<PathBuf> =
//     LazyLock::new(|| internal_storage_dir().expect("Failed to get internal storage dir"));
// #[cfg(target_os = "android")]
// fn internal_storage_dir() -> anyhow::Result<PathBuf> {
//     fn run(env: &mut JNIEnv<'_>, activity: &JObject<'_>) -> anyhow::Result<PathBuf> {
//         let files_dir = env
//             .call_method(activity, "getFilesDir", "()Ljava/io/File;", &[])?
//             .l()?;
//         let files_dir: JString<'_> = env
//             .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])?
//             .l()?
//             .into();
//         let files_dir: String = env.get_string(&files_dir)?.into();
//         Ok(PathBuf::from(files_dir))
//     }

//     let (tx, rx) = std::sync::mpsc::channel();
//     dioxus::mobile::wry::prelude::dispatch(move |env, activity, _webview| {
//         tx.send(run(env, activity)).unwrap()
//     });
//     rx.recv().unwrap()
// }


/// Constructs the full path to the settings file for a given key.
/// Files are stored in a platform-specific application data directory.
fn get_settings_file_path(key: &str) -> io::Result<PathBuf> {
    #[cfg(target_os = "android")]
    {
        // On Android, use a hardcoded path within the app's sandboxed data directory.
        // This is a common practice for Dioxus mobile apps.
        let mut path = PathBuf::from("/data/data/com.yourcompany.yourapp/files");
        fs::create_dir_all(&path)?;
        path.push(format!("{}.json", key));
        Ok(path)
    }
    #[cfg(not(target_os = "android"))]
    {
     
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
