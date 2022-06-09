use std::{fs::File, io::Write, os::unix::prelude::PermissionsExt, path::PathBuf};

use crate::install::{APP_ICON, APP_NAME, BINARY_NAME};

pub fn generate_shortcut(path: PathBuf, version: String, description: String) {
    let mut path_shortcut = path.clone();
    path_shortcut.push(format!("{}.desktop", BINARY_NAME));
    println!("{}", &path_shortcut.to_str().unwrap());
    let mut file_shortcut = File::create(&path_shortcut).unwrap();
    file_shortcut
        .write_all(generate_content(version, description).as_bytes())
        .unwrap();
    let mut perms = path_shortcut.metadata().unwrap().permissions();
    perms.set_mode(0o755); // make this file executable
    std::fs::set_permissions(path_shortcut, perms).unwrap();
}

fn generate_content(version: String, description: String) -> String {
    format!(
        r#"[Desktop Entry]
Version={}
Description={}
Type=Application
Name={}
Exec={}
Icon={}"#,
        version, description, APP_NAME, BINARY_NAME, APP_ICON
    )
}

mod test {
    use super::generate_content;

    #[test]
    fn shortcut_verify_content() {
        let result = r#"[Desktop Entry]
Version=1.0.1
Description=Test Description
Type=Application
Name=Adb GUI
Exec=adbgui
Icon=adbgui"#;
        assert_eq!(
            generate_content("1.0.1".to_string(), "Test Description".to_string()).as_str(),
            result
        )
    }
}
