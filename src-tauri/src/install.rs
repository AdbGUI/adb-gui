use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::utils::unzip_file;
use dirs::{data_local_dir, desktop_dir, executable_dir};
use futures_util::stream::StreamExt;
use reqwest::Client;

pub struct Tool<'a>(&'a str, &'a str);

const APP_NAME: &str = "ADB GUI";
const BINARY_NAME: &str = "adbgui";
const AUTHOR: &str = "SergioRibera";
const TOOLS: [Tool; 2] = [
    Tool(
        "adb",
        "https://dl.google.com/android/repository/platform-tools-latest-{so}.zip",
    ),
    Tool("scrcpy", ""),
];

#[derive(Clone)]
pub struct Installer {
    app_root_dir: String,
    version: String,
    description: String,
    desktop_shortcut: bool,
    menu_shortcut: bool,
}

impl Installer {
    pub fn new(vers: String) -> Self {
        Self {
            app_root_dir: String::from(get_default_install_path().unwrap().to_str().unwrap()),
            version: vers,
            description: String::from("GUI for the commons and more necesary tools on adb"),
            desktop_shortcut: true,
            menu_shortcut: true,
        }
    }
    pub fn set_install_path(&mut self, path: String) { self.app_root_dir = path; }
    pub fn set_version(&mut self, version: String) { self.version = version; }
    pub fn set_desktop_shortcut(&mut self, shortcut: bool) { self.desktop_shortcut = shortcut; }
    pub fn set_menu_shortcut(&mut self, shortcut: bool) { self.menu_shortcut = shortcut; }

    pub fn is_installed(&self) -> bool {
        let conf_path = env::current_exe().unwrap().join("conf.json");
        conf_path.exists()
    }

    pub fn get_installed_path(&self) -> PathBuf {
        if self.is_installed() {
            return env::current_exe().unwrap();
        }
        get_default_install_path().unwrap()
    }

    pub fn set_env_vars(&self) {
        let mut path = env::var("PATH").unwrap_or(String::new());
        path.push_str(format!(":{}", self.app_root_dir).as_str());
        env::set_var("PATH", path);
    }

    pub async fn install_tools(
        &self,
        client: Client,
        progress: &mut u64,
        progress_tool: impl Fn(String, u64),
    ) {
        // download chunks
        for tool in TOOLS.iter() {
            let url = tool.1.replace("{so}", &get_so_name());
            let res = client
                .get(&url)
                .send()
                .await
                .or(Err(format!("Failed to GET from '{}'", &url)))
                .unwrap();
            let total_size = res
                .content_length()
                .ok_or(format!("Failed to get content length from '{}'", &url))
                .unwrap();
            let (tool_name, tool_url) = (&tool.0, &tool.1);
            let tool_dest_path = Path::new(&self.app_root_dir).join(&tool_name);
            let tool_dest_path_str = tool_dest_path.as_path().to_str().unwrap();
            let tool_tmp_path = Path::new(&self.app_root_dir).join(format!("{}.zip", &tool_name));
            if !tool_tmp_path.exists() {
                let file_downloaded = match File::create(&tool_tmp_path) {
                    Err(why) => panic!("couldn't create {}", why),
                    Ok(mut file) => {
                        let mut downloaded: u64 = 0;
                        let mut stream = res.bytes_stream();

                        while let Some(item) = stream.next().await {
                            let chunk = item
                                .or(Err(format!("Error while downloading file")))
                                .unwrap();
                            file.write(&chunk)
                                .or(Err(format!("Error while writing to file")))
                                .unwrap();
                            let new = min(downloaded + (chunk.len() as u64), total_size);
                            downloaded = new;
                            progress_tool(format!("Downloading {}...", tool_name.to_string()), new);
                        }
                        file
                    }
                };
                unzip_file(file_downloaded, &tool_dest_path_str).unwrap();
            }
            progress_tool(format!("{} downloaded!", &tool_name), *progress);
        }
    }

    pub async fn install(&self, progress_install: impl Fn(String, u64)) {
        let client = Client::new();
        let app_dir = Path::new(&self.app_root_dir);
        let mut progress: u64 = 0;
        // Calcule total size
        let mut max_progress: u64 = 0;
        for tool in TOOLS.iter() {
            let url = tool.1;
            let res = client
                .get(url)
                .send()
                .await
                .or(Err(format!("Failed to GET from '{}'", &url)))
                .unwrap();
            let total_size = res
                .content_length()
                .ok_or(format!("Failed to get content length from '{}'", &url))
                .unwrap();
            max_progress = max_progress + total_size;
        }

        // Create directory
        if !&app_dir.exists() {
            std::fs::create_dir_all(&app_dir).unwrap();
            progress_install("Creating directory...".to_string(), progress);
            progress = progress + 1;
        }

        // Copy binary
        match env::current_exe() {
            Ok(exe_path) => {
                std::fs::copy(exe_path, app_dir.join(BINARY_NAME));
                progress_install("Copying Binary...".to_string(), progress);
                progress = progress + 1;
                if !cfg!(target_os = "windows") {
                    self.set_env_vars();
                    progress_install("Set Enviroment Variables...".to_string(), progress);
                    progress = progress + 1;
                }
            }
            Err(_) => println!("Error copying binary"),
        }

        // Install tools
        self.install_tools(client, &mut progress, &progress_install);

        // Create conf.json
        let conf_path = app_dir.join("conf.json");
        if !conf_path.exists() {
            let conf = Config::new();
            let conf_str = serde_json::to_string(&conf).unwrap();
            let mut file = File::create(&conf_path).unwrap();
            file.write_all(conf_str.as_bytes()).unwrap();
            progress_install("Creating Default Configurations...".to_string(), progress);
            progress = progress + 1;
        }

        // Generate shortcuts
        if self.desktop_shortcut || self.menu_shortcut {
            self.generate_shortcut(
                self.version.clone(),
                self.description.clone(),
                self.desktop_shortcut,
                self.menu_shortcut,
            );
            progress_install("Generating shortcuts...".to_string(), progress);
        }
    }

    pub fn generate_shortcut(
        &self,
        version: String,
        description: String,
        on_desktop: bool,
        on_startup_menu: bool,
    ) {
        let path = self.get_installed_path();
        #[cfg(target_os = "windows")]
        {
            let mut path_shortcut = path.clone();
            path_shortcut.push("adbgui.lnk");
            std::os::windows::fs::symlink_file(path.join("adbgui.exe"), &path_shortcut);
        }
        #[cfg(target_os = "linux")]
        {
            let mut path_shortcut = path.clone();
            path_shortcut.push("adbgui.desktop");
            let mut file_shortcut = File::create(&path_shortcut).unwrap();
            file_shortcut
                .write_all(
                    format!(
                        r#"
                [Desktop Entry]
                Version={}
                Description={}
                Type=Application
                Name={}
                Exec={}
                Icon={}
                "#,
                        version, APP_NAME, description, BINARY_NAME, BINARY_NAME
                    )
                    .as_bytes(),
                )
                .unwrap();
            let mut perms = path_shortcut.metadata().unwrap().permissions();
            perms.set_mode(0o755); // make this file executable
            std::fs::set_permissions(path_shortcut, perms).unwrap();
        }
        #[cfg(target_os = "macos")]
        {
            let mut path_shortcut = path.clone();
            path_shortcut.push("adbgui.app");
            let file_shortcut = File::create(&path_shortcut).unwrap();
            file_shortcut
                .write_all(
                    format!(
                        r#"
                #!/bin/bash
                ## This file is created automatically by ADB GUI installer
                export EXE={exe:s}
                export SCRIPT={script:s}
                export ARGS='{args:s}'
                ## Execute application
                $EXE $SCRIPT $ARGS
                "#,
                        version, APP_NAME, description, BINARY_NAME, BINARY_NAME
                    )
                    .as_bytes(),
                )
                .unwrap();

            let mut perms = path_shortcut.metadata().unwrap().permissions();
            perms.set_mode(0o755); // make this file executable
            std::fs::set_permissions(path_shortcut, perms).unwrap();
        }
    }
}

pub fn get_so_name() -> String {
    if cfg!(target_os = "windows") {
        return String::from("windows");
    } else if cfg!(target_os = "macos") {
        return String::from("macos");
    }
    String::from("linux")
}

pub fn get_default_install_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    return Some(
        data_local_dir()
            .unwrap()
            .join(AUTHOR)
            .join(APP_NAME.replace(" ", "_")),
    );

    // TODO: Add this path into enviroment variables

    // Another than not working on windows
    return Some(
        executable_dir()
            .unwrap()
            .join(AUTHOR)
            .join(APP_NAME.replace(" ", "_")),
    );
}

pub fn get_install_path_tool() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    return Some(
        data_local_dir()
            .unwrap()
            .join(AUTHOR)
            .join(APP_NAME.replace(" ", "_")),
    );

    // Another than not working on windows
    return Some(
        executable_dir()
            .unwrap()
            .join(AUTHOR)
            .join(APP_NAME.replace(" ", "_")),
    );
}
