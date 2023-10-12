// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

#[tauri::command]
fn create_project(serial: &str, name: &str, path: &str) -> String {
    if serial.len() == 0 || name.len() == 0 || path.len() == 0 {
        return "Err: Serial, name and path cannot be empty!".to_string();
    }

    let project_name = name
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let project_path = format!("{}\\{}-{}", path, serial, project_name);

    fs::create_dir_all(&project_path).unwrap();

    let unity_path = format!("{}\\Source-Unity-{serial}-{project_name}", project_path);
    fs::create_dir_all(&unity_path).unwrap();

    fs::create_dir_all(format!("{}\\Project", unity_path)).unwrap();
    let project_keep_path = format!("{}\\Project\\.keep", unity_path);
    if !Path::new(&project_keep_path).exists() {
        File::create(&project_keep_path).unwrap();
    }

    fs::create_dir_all(format!("{}\\Package", unity_path)).unwrap();
    let package_keep_path = format!("{}\\Package\\.keep", unity_path);
    if !Path::new(&package_keep_path).exists() {
        File::create(&package_keep_path).unwrap();
    }

    let git_path = format!("{}\\.git", unity_path);
    if !Path::new(&git_path).exists() {
        let output = Command::new("git")
            .arg("init")
            .current_dir(&unity_path)
            .output()
            .unwrap();

        if !output.status.success() {
            return "Err: Failed to init git".to_string();
        }
    }

    fs::create_dir_all(format!("{}\\Deploy", project_path)).unwrap();

    if let Err(e) = std::process::Command::new("explorer")
        .arg(&project_path)
        .spawn()
    {
        eprintln!("Failed to open explorer: {}", e);
    }

    format!("Project created at {}", project_path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
