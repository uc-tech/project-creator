// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, File};
use std::process::Command;

#[tauri::command]
fn create_project(serial: &str, name: &str, path: &str) -> String {
  if serial.len() == 0 || name.len() == 0 || path.len() == 0 {
    format!("")
  } else {
    let n: String = name.chars().filter(|c| !c.is_whitespace()).collect();
    let p: String = format!("{}\\{}-{}", path, serial, n);
    if fs::create_dir_all(p.clone()).is_err() {
      format!("");
    }

    let unity_path: String = format!("{}\\Source-Unity-{serial}-{n}", p);
    fs::create_dir_all(unity_path.clone()).unwrap();
    fs::create_dir_all(format!("{}\\Project", unity_path)).unwrap();
    File::create(format!("{}\\Project\\.keep", unity_path)).unwrap();
    fs::create_dir_all(format!("{}\\Package", unity_path)).unwrap();
    File::create(format!("{}\\Package\\.keep", unity_path)).unwrap();

    fs::create_dir_all(format!("{}\\Deploy", p)).unwrap();

    Command::new( "explorer" )
        .arg( p.clone() )
        .spawn( )
        .unwrap( );
    format!("Hello! {}", p)
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![create_project])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
