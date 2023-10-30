// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{collections::HashMap, fs};

// fn list_directories_in_path(directory_path: &str) -> Result<Vec<String>, std::io::Error> {
//     let mut directory_names = Vec::new();

//     // Use std::fs::read_dir to get a list of directory entries
//     let entries = fs::read_dir(directory_path)?;

//     for entry in entries {
//         if let Ok(entry) = entry {
//             if let Some(file_name) = entry.file_name().to_str() {
//                 directory_names.push(file_name.to_string());
//             }
//         }
//     }

//     Ok(directory_names)
// }
fn list_all_entries_in_directory(
    directory_path: &str,
) -> Result<Vec<HashMap<String, String>>, std::io::Error> {
    let mut entries_info = Vec::new();

    // Use std::fs::read_dir to get a list of directory entries
    let read_dir = fs::read_dir(directory_path)?;

    for entry in read_dir {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            let mut entry_info = HashMap::new();

            if entry_path.is_dir() {
                entry_info.insert("type".to_string(), "folder".to_string());
            } else {
                entry_info.insert("type".to_string(), "file".to_string());
            }

            entry_info.insert(
                "name".to_string(),
                entry.file_name().to_string_lossy().to_string(),
            );
            entries_info.push(entry_info);
        }
    }

    Ok(entries_info)
}
#[tauri::command]
fn return_cdrive(the_path: &str) -> Vec<HashMap<String, String>> {
    let str_list = list_all_entries_in_directory(the_path);
    match str_list {
        Ok(strings) => strings,
        Err(err) => {
            let mut hs = HashMap::new();
            hs.insert("error".to_string(), err.to_string());
            vec![hs]
        }
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![return_cdrive])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
