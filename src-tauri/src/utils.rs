#[tauri::command]
pub fn open_explorer() {
    #[cfg(target_os = "linux")] {
        std::process::Command::new("xdg-open")
        .arg("./characters")
        .spawn()
        .expect("Failed to open file explorer");
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
        .arg(".\\characters")
        .spawn()
        .expect("Failed to open file explorer");
    }

}