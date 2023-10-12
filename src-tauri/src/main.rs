// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, File};
use regex::Regex;

#[tauri::command]
fn create_project(serial: &str, name: &str, path: &str, gitignore: &str, gitattributes: &str) -> Result<String, String> {
    if serial.len() == 0 || name.len() == 0 || path.len() == 0 {
        return Err("Serial, name and path cannot be empty!".into());
    }
    
    let project_name: String;
    match Regex::new(r"[^a-zA-Z0-9_]") {
        Ok(re) => {
            project_name = re.replace_all(&name, "").to_string();
        }
        Err(e) => {
            return Err(format!("Fail to parse project name: {}", e));
        }
    }
    let root_path = format!("{}\\{}-{}", path, serial, project_name);

    if let Err(e) = fs::create_dir_all(&root_path) {
        return Err(format!("Fail to create project path: {}", e));
    }

    let unity_path = format!("{}\\Source-Unity-{}-{}", root_path, serial, project_name);
    if let Err(e) = fs::create_dir(&unity_path) {
        return Err(format!("Fail to create unity source path: {}", e));
    }

    let project_path = format!("{}\\Project", unity_path);
    if let Err(e) = fs::create_dir(&project_path) {
        return Err(format!("Failed to create dir: {}", e));
    }
    let project_keep_path = format!("{}\\.keep", project_path);
    if let Err(e) = File::create(&project_keep_path) {
        return Err(format!("Failed to create file: {}", e));
    }

    let package_path = format!("{}\\Package", unity_path);
    if let Err(e) = fs::create_dir(&package_path) {
        return Err(format!("Failed to create dir: {}", e));
    }
    /* 
    let package_keep_path = format!("{}\\.keep", package_path);
    if let Err(err) = File::create(&package_keep_path) {
        return Err(err.to_string());
    }
    */

    let proj_pkg_path = format!("{}\\UltraCombos.{}", package_path, project_name);
    if let Err(e) = fs::create_dir(&proj_pkg_path) {
        return Err(format!("Failed to create dir: {}", e));
    }
    let proj_pkg_json = format!("{}\\package.json", proj_pkg_path);
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
    if let Err(e) = fs::write(&proj_pkg_json, json) {
        return Err(format!("Failed to write file: {}", e));
    }

    let proj_pkg_runtime_path = format!("{}\\Runtime", proj_pkg_path);
    if let Err(e) = fs::create_dir(&proj_pkg_runtime_path) {
        return Err(format!("Failed to create dir: {}", e));
    }
    let proj_pkg_runtime_file = format!("{}\\UltraCombos.{}.Runtime.asmdef", proj_pkg_runtime_path, project_name);
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
    if let Err(e) = fs::write(&proj_pkg_runtime_file, content) {
        return Err(format!("Failed to write file: {}", e));
    }

    let proj_pkg_readme_file = format!("{}\\README.md", proj_pkg_path);
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
    if let Err(e) = fs::write(&proj_pkg_readme_file, readme) {
        return Err(format!("Failed to write file: {}", e));
    }

    let proj_pkg_gitignore_file = format!("{}\\.gitignore", proj_pkg_path);
    if let Err(e) = fs::write(&proj_pkg_gitignore_file, gitignore) {
        return Err(format!("Failed to write file: {}", e));
    }

    let proj_pkg_gitattributes_file = format!("{}\\.gitattributes", proj_pkg_path);
    if let Err(e) = fs::write(&proj_pkg_gitattributes_file, gitattributes) {
        return Err(format!("Failed to write file: {}", e));
    }

    if let Err(e) = std::process::Command::new("git")
        .arg("init")
        .current_dir(&unity_path)
        .spawn()
    {
        return Err(format!("Failed to init git: {}", e));
    }

    if let Err(e) = fs::create_dir(format!("{}\\Deploy", root_path)) {
        return Err(format!("Failed to create dir: {}", e));
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
