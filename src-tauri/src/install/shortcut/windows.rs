use std::path::PathBuf;

use crate::install::{APP_ICON, APP_NAME, BINARY_NAME};

pub fn generate_shortcut(path: PathBuf, version: String, description: String) {
    let mut path_shortcut = path.clone();
    path_shortcut.push(generate_file("lnk"));
    std::os::windows::fs::symlink_file(path.join(generate_file("exe")), &path_shortcut);
}

fn generate_file (ext: &str) -> &str {
    format!("{}.{}", BINARY_NAME, ext).as_str()
}

mod test {
    #[test]
    fn verify_generate_lnk() {
        let result = format!("{}.lnk", BINARY_NAME).as_str();

        assert_eq!(generate_file("lnk"), result);
    }

    fn verify_generate_exe() {
        let result = format!("{}.exe", BINARY_NAME).as_str();

        assert_eq!(generate_file("exe"), result);
    }
}
