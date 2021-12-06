use std::sync::{Arc, Mutex};

use tauri::{Manager, State, Window, generate_context, generate_handler};

mod utils;
mod install;
mod config;
use install::Installer;

struct InstallState(Arc<Mutex<Installer>>);
impl InstallState {
    pub fn new(version: String) -> Self {
        Self {
            0: Arc::new(Mutex::new(Installer::new(version)))
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
fn close_splashscreen(window: Window, installer_state: State<'_, InstallState>) {
    if let Some(splashscreen) = window.get_window("splashscreen") {
        println!("Closing splashscreen");
        splashscreen.close().unwrap();
        // Show main window
        if !installer_state.0.lock().unwrap().is_installed() {
            println!("Showing install window");
            if let Some(install_screen) = window.get_window("install") {
                install_screen.show().unwrap();
                install_screen.emit_to("install", "install-path", installer_state.0.lock().unwrap().get_installed_path().to_str()).unwrap();
            }
        } else {
            println!("Showing main window");
            window.get_window("main").unwrap().show().unwrap();
        }
    }
}

// Event to set path to install
#[allow(dead_code)]
#[tauri::command]
fn set_install_path(installer_state: State<'_, InstallState>, path: String) {
    println!("Path to install: {:?}", path);
    installer_state.0.lock().unwrap().set_install_path(path);
}

// Event to set shortcut on desktop
#[allow(dead_code)]
#[tauri::command]
fn set_desktop_shortcut(installer_state: State<'_, InstallState>, shortcut: bool) {
    println!("Desktop Shortcut: {:?}", shortcut);
    installer_state.0.lock().unwrap().set_desktop_shortcut(shortcut);
}

// Event to set shortcut on menu
#[allow(dead_code)]
#[tauri::command]
fn set_menu_shortcut(installer_state: State<'_, InstallState>, shortcut: bool) {
    println!("Menu Shortcut: {:?}", shortcut);
    installer_state.0.lock().unwrap().set_menu_shortcut(shortcut);
}

fn main() {
    let context = generate_context!();
    let app_version = &context.config().package.version;

    tauri::Builder::default()
        .manage(InstallState::new(app_version.as_ref().unwrap().to_string()))
        .setup(|_app| {
            Ok(())
        })
        .invoke_handler(
            generate_handler![
            close_splashscreen,
            set_install_path,
            set_menu_shortcut,
            set_desktop_shortcut
        ])
        .run(context)
        .expect("error while running tauri application");
}
