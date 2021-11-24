mod install;
use install::*;

use tauri::Manager;

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            let install_window = app.get_window("install").unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // initialize your app here instead of sleeping :)
                println!("Initializing...");
                std::thread::sleep(std::time::Duration::from_secs(2));
                println!("Done initializing.");

                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                if verify_installed_tools() {
                    main_window.show().unwrap();
                } else {
                    install_window.show().unwrap();
                    install_window.emit("install-path", get_install_path().unwrap().to_str()).unwrap();
                }
            });
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
