use std::path::PathBuf;

use dirs::{ data_local_dir, executable_dir };

const APP_NAME: &str = "ADB GUI";
const AUTHOR: &str = "SergioRibera";
const TOOLS: [&'static str; 2] = ["adb", "scrcpy"];

pub fn verify_installed_tools() -> bool {
    let mut installed = true;
    for tool in TOOLS.iter() {
        let path = get_install_path_tool().unwrap().join(tool);
        if !path.exists() {
            installed = false;
            println!("{} is not installed", tool);
        }
    }
    installed
}

pub fn get_install_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    return Some(data_local_dir().unwrap().join(AUTHOR).join(APP_NAME.replace(" ", "_")));
    
    // TODO: Add this path into enviroment variables

    // Another than not working on windows
    return Some(executable_dir().unwrap().join(AUTHOR).join(APP_NAME.replace(" ", "_")));
}

pub fn get_install_path_tool() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    return Some(data_local_dir().unwrap().join(AUTHOR).join(APP_NAME.replace(" ", "_")));
    
    // Another than not working on windows
    return Some(executable_dir().unwrap().join(AUTHOR).join(APP_NAME.replace(" ", "_")));
}

pub fn  generate_shortcut(on_desktop: bool, on_startup_menu: bool) {
    let path = get_install_path().unwrap();
    #[cfg(target_os = "windows")]
    {
        let mut path_shortcut = path.clone();
        path_shortcut.push("adbgui.lnk");
    }
    #[cfg(target_os = "linux")]
    {
        let mut path_shortcut = path.clone();
        path_shortcut.push("adbgui.desktop");
    }
    #[cfg(target_os = "macos")]
    {
        let mut path_shortcut = path.clone();
        path_shortcut.push("adbgui.app");
    }
}
