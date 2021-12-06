use tauri::{Manager, State, Window, generate_context, generate_handler};

mod utils;
mod install;
mod config;
use install::Installer;


#[allow(dead_code)]
#[tauri::command]
fn close_splashscreen(window: Window, installer_state: State<'_, Installer>) {
    if let Some(splashscreen) = window.get_window("splashscreen") {
        println!("Closing splashscreen");
        splashscreen.close().unwrap();
        // Show main window
        if !installer_state.is_installed() {
            println!("Showing install window");
            if let Some(install_screen) = window.get_window("install") {
                install_screen.show().unwrap();
                install_screen.emit_to("install", "install-path", installer_state.get_installed_path().to_str()).unwrap();
            }
        } else {
            println!("Showing main window");
            window.get_window("main").unwrap().show().unwrap();
        }
    }
}

fn main() {
    let context = generate_context!();
    let _app_settings = &context.config().package.version;

    println!("{:?}", _app_settings);
    tauri::Builder::default()
        .manage(Installer::new())
        .setup(|_app| {
            Ok(())
        })
        .invoke_handler(generate_handler![close_splashscreen])
        .run(context)
        .expect("error while running tauri application");
}
