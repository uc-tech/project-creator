// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, File};

#[tauri::command]
fn create_project(serial: &str, name: &str, path: &str) -> Result<String, String> {
    if serial.len() == 0 || name.len() == 0 || path.len() == 0 {
        return Err("Serial, name and path cannot be empty!".into());
    }

    let project_name = name
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let root_path = format!("{}\\{}-{}", path, serial, project_name);

    if let Err(e) = fs::create_dir_all(&root_path) {
        return Err(format!("Create project path: {}", e));
    }

    let unity_path = format!("{}\\Source-Unity-{}-{}", root_path, serial, project_name);
    if let Err(e) = fs::create_dir(&unity_path) {
        return Err(format!("Create unity source path: {}", e));
    }

    let project_path = format!("{}\\Project", unity_path);
    if let Err(err) = fs::create_dir(&project_path) {
        return Err(err.to_string());
    }
    let project_keep_path = format!("{}\\.keep", project_path);
    if let Err(err) = File::create(&project_keep_path) {
        return Err(err.to_string());
    }

    let package_path = format!("{}\\Package", unity_path);
    if let Err(err) = fs::create_dir(&package_path) {
        return Err(err.to_string());
    }
    /* 
    let package_keep_path = format!("{}\\.keep", package_path);
    if let Err(err) = File::create(&package_keep_path) {
        return Err(err.to_string());
    }
    */

    let proj_pkg_path = format!("{}\\UltraCombos.{}", package_path, project_name);
    if let Err(err) = fs::create_dir(&proj_pkg_path) {
        return Err(err.to_string());
    }
    let proj_pkg_json = format!("{}\\package.json", proj_pkg_path);
    if let Err(err) = File::create(&proj_pkg_json) {
        return Err(err.to_string());
    }
    let package_name = format!("com.ultracombos.{}", project_name.to_lowercase());
    let json = format!(
        r#"{{
    "name": "{package_name}",
    "version": "1.0.0",
    "displayName": "{name}",
    "description": "Unity Package for {name}",
    "type": "library",
    "author": {{
        "name": "Ultra Combos Co., Ltd.",
        "email": "tech@ultracombos.com",
        "url": "https://ultracombos.com"
    }}
}}"#
    );
    if let Err(err) = fs::write(&proj_pkg_json, json) {
        return Err(err.to_string());
    }

    let proj_pkg_runtime_path = format!("{}\\Runtime", proj_pkg_path);
    if let Err(err) = fs::create_dir(&proj_pkg_runtime_path) {
        return Err(err.to_string());
    }
    let proj_pkg_runtime_file = format!("{}\\UltraCombos.{}.Runtime.asmdef", proj_pkg_runtime_path, project_name);
    if let Err(err) = File::create(&proj_pkg_runtime_file) {
        return Err(err.to_string());
    }
    let content = format!(
        r#"{{
    "name": "UltraCombos.{project_name}.Runtime",
    "rootNamespace": "UltraCombos.{project_name}",
    "references": [],
    "includePlatforms": [],
    "excludePlatforms": [],
    "allowUnsafeCode": false,
    "overrideReferences": false,
    "precompiledReferences": [],
    "autoReferenced": true,
    "defineConstraints": [],
    "versionDefines": [],
    "noEngineReferences": false
}}"#
    );
    if let Err(err) = fs::write(&proj_pkg_runtime_file, content) {
        return Err(err.to_string());
    }

    let proj_pkg_readme_file = format!("{}\\README.md", proj_pkg_path);
    if let Err(err) = File::create(&proj_pkg_readme_file) {
        return Err(err.to_string());
    }
    let readme = format!(
        r#"## Unity Package for {name}

```json
{{
    "dependencies": {{
        "{package_name}": "file:<project>/../../../../Package/UltraCombos.{project_name}"
    }}
}}
```
        "#
    );
    if let Err(err) = fs::write(&proj_pkg_readme_file, readme) {
        return Err(err.to_string());
    }

    if let Err(e) = std::process::Command::new("git")
        .arg("init")
        .current_dir(&unity_path)
        .spawn()
    {
        return Err(format!("Failed to init git: {}", e));
    }

    if let Err(err) = fs::create_dir(format!("{}\\Deploy", root_path)) {
        return Err(err.to_string());
    }

    if let Err(e) = std::process::Command::new("explorer")
        .arg(&root_path)
        .spawn()
    {
        return Err(format!("Failed to open explorer: {}", e));
    }

    Ok(format!("Project created at {}", root_path))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
