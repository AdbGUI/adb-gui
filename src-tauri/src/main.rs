use std::sync::{Arc, Mutex};
use tauri::{Manager, Position, State, Window, generate_context};

mod utils;
mod install;
mod config;
use install::Installer;

// wrappers around each Window
// we use a dedicated type because Tauri can only manage a single instance of a given type
struct SplashscreenWindow(Arc<Mutex<Window>>);
struct MainWindow(Arc<Mutex<Window>>);

#[allow(dead_code)]
#[tauri::command]
fn close_splashscreen( _: Window, splashscreen: State<SplashscreenWindow>, main: State<MainWindow>,) {
    // Close splashscreen
    splashscreen.0.lock().unwrap().close().unwrap();
    // Show main window
    main.0.lock().unwrap().show().unwrap();
}

fn main() {
    let context = generate_context!();
    let _app_settings = &context.config().package.version;

    tauri::Builder::default()
        .setup(|app| {
            // set the splashscreen and main windows to be globally available with the tauri state API
            app.manage(SplashscreenWindow(Arc::new(Mutex::new(
                app.get_window("splashscreen").unwrap(),
            ))));
            app.manage(MainWindow(Arc::new(Mutex::new(
                app.get_window("main").unwrap(),
            ))));

            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
