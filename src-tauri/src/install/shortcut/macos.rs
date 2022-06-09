use std::{fs::File, io::Write, os::unix::prelude::PermissionsExt, path::PathBuf};

use crate::install::{APP_ICON, APP_NAME, BINARY_NAME};

pub fn generate_shortcut(path: PathBuf, version: String, description: String) {
    let mut path_shortcut = path.clone();
    path_shortcut.push(format!("{}.app", BINARY_NAME));
    let file_shortcut = File::create(&path_shortcut).unwrap();
    file_shortcut
        .write_all(generate_content(version, description).as_bytes())
        .unwrap();

    let mut perms = path_shortcut.metadata().unwrap().permissions();
    perms.set_mode(0o755); // make this file executable
    std::fs::set_permissions(path_shortcut, perms).unwrap();
}

fn generate_content(version: String, description: String) -> String {
    format!(
r#"#!/bin/bash
## This file is created automatically by ADB GUI installer
export EXE={}
export SCRIPT=''
export ARGS=''
## Execute application
$EXE $SCRIPT $ARGS"#,
        BINARY_NAME
    )
}

mod test {
    use super::generate_content;

    #[test]
    fn shortcut_verify_content() {
        let result =
r#"#!/bin/bash
## This file is created automatically by ADB GUI installer
export EXE=adbgui
export SCRIPT=''
export ARGS=''
## Execute application
$EXE $SCRIPT $ARGS"#;
        assert_eq!(
            generate_content("1.0.1".to_string(), "Test Description".to_string()).as_str(),
            result
        )
    }
}
