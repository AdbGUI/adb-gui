use tauri::{Manager, Window, generate_context, generate_handler};

mod utils;
mod install;
mod config;
use install::Installer;


#[allow(dead_code)]
#[tauri::command]
fn close_splashscreen(window: Window) {
    if let Some(splashscreen) = window.get_window("splashscreen") {
        println!("Closing splashscreen");
        splashscreen.close().unwrap();
        // Show main window
        window.get_window("install").unwrap().show().unwrap();
    }
}

fn main() {
    let context = generate_context!();
    let _app_settings = &context.config().package.version;

    println!("{:?}", _app_settings);
    tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
    .invoke_handler(generate_handler![close_splashscreen])
        .run(context)
        .expect("error while running tauri application");
}
