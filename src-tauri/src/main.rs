use std::sync::Arc;

use serde::Serialize;
use tauri::{generate_context, generate_handler, Manager, State, Window};
use tokio::sync::Mutex;

mod config;
mod install;
mod tools;
mod utils;
use install::Installer;

struct InstallState(Arc<Mutex<Installer>>);
impl InstallState {
    pub fn new(version: String) -> Self {
        Self {
            0: Arc::new(Mutex::new(Installer::new(version))),
        }
    }
}

#[allow(dead_code)]
#[tauri::command]
async fn close_splashscreen(
    window: Window,
    installer_state: State<'_, InstallState>,
) -> Result<(), ()> {
    if let Some(splashscreen) = window.get_window("splashscreen") {
        println!("Closing splashscreen");
        splashscreen.close().unwrap();
        // Show main window
        if !installer_state.0.lock().await.is_installed() {
            println!("Showing install window");
            if let Some(install_screen) = window.get_window("install") {
                install_screen.show().unwrap();
                install_screen
                    .emit_to(
                        "install",
                        "install-path",
                        installer_state.0.lock().await.get_installed_path().to_str(),
                    )
                    .unwrap();
            }
        } else {
            println!("Showing main window");
            window.get_window("main").unwrap().show().unwrap();
        }
    }
    Ok(())
}

// Event to set path to install
#[allow(dead_code)]
#[tauri::command]
async fn set_install_path(
    installer_state: State<'_, InstallState>,
    path: String,
) -> Result<(), ()> {
    println!("Path to install: {:?}", path);
    installer_state.0.lock().await.set_install_path(path);
    Ok(())
}

// Event to set shortcut on desktop
#[allow(dead_code)]
#[tauri::command]
async fn set_desktop_shortcut(
    installer_state: State<'_, InstallState>,
    shortcut: bool,
) -> Result<(), ()> {
    println!("Desktop Shortcut: {:?}", shortcut);
    installer_state
        .0
        .lock()
        .await
        .set_desktop_shortcut(shortcut);
    Ok(())
}

// Event to set shortcut on menu
#[allow(dead_code)]
#[tauri::command]
async fn set_menu_shortcut(
    installer_state: State<'_, InstallState>,
    shortcut: bool,
) -> Result<(), ()> {
    println!("Menu Shortcut: {:?}", shortcut);
    installer_state.0.lock().await.set_menu_shortcut(shortcut);
    Ok(())
}

// Event to start install process
#[allow(dead_code)]
#[tauri::command]
async fn install(installer_state: State<'_, InstallState>, window: Window) -> Result<(), String> {
    println!("Installing...");
    let install_windows = window.get_window("install").unwrap();
    let mut installer = installer_state.0.lock().await;
    if !installer.installing {
        installer
            .install(move |msg| {
                println!("Installing...{:?}", msg);
                install_windows
                    .emit_to(
                        "install",
                        "install-progress",
                        msg,
                    )
                    .unwrap();
            })
            .await;
    }
    Ok(())
}

fn main() {
    let context = generate_context!();
    let app_version = &context.config().package.version;

    tauri::Builder::default()
        .manage(InstallState::new(app_version.as_ref().unwrap().to_string()))
        .setup(|_app| Ok(()))
        .invoke_handler(generate_handler![
            close_splashscreen,
            set_install_path,
            set_menu_shortcut,
            set_desktop_shortcut,
            install
        ])
        .run(context)
        .expect("error while running tauri application");
}
