use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::utils::{unzip_file, get_so_name};
use dirs::{data_local_dir, desktop_dir, executable_dir};
use futures_util::stream::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

mod shortcut;
use shortcut::generate_shortcut;

pub struct Tool<'a>(&'a str, &'a str);

const APP_NAME: &str = "Adb GUI";
const APP_ICON: &str = "adbgui";
const BINARY_NAME: &str = "adbgui";
const AUTHOR: &str = "SergioRibera";
const TOOLS: [Tool; 2] = [
    Tool(
        "adb",
        "https://dl.google.com/android/repository/platform-tools-latest-{so}.zip",
    ),
    Tool(
        "scrcpy",
        "https://dl.google.com/android/repository/platform-tools-latest-{so}.zip",
    ),
];

#[derive(Clone, Serialize, Deserialize)]
pub struct Installer {
    pub installing: bool,
    app_root_dir: String,
    version: String,
    description: String,
    desktop_shortcut: bool,
    menu_shortcut: bool,
}

impl Installer {
    pub fn new(vers: String) -> Self {
        Self {
            installing: false,
            app_root_dir: String::from(get_default_install_path().unwrap().to_str().unwrap()),
            version: vers,
            description: String::from("GUI for the commons and more necesary tools on adb"),
            desktop_shortcut: true,
            menu_shortcut: true,
        }
    }
    pub fn set_install_path(&mut self, path: String) { self.app_root_dir = format!("{}/{}/{}", path, AUTHOR, APP_NAME); }
    pub fn set_desktop_shortcut(&mut self, shortcut: bool) { self.desktop_shortcut = shortcut; }
    pub fn set_menu_shortcut(&mut self, shortcut: bool) { self.menu_shortcut = shortcut; }
    pub fn set_installing(&mut self, installing: bool) { self.installing = installing; }

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
        progress_tool: impl Fn(String),
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
            let tool_name = &tool.0;
            let tool_dest_path = Path::new(&self.app_root_dir).join(&tool_name);
            let tool_dest_path_str = tool_dest_path.as_path().to_str().unwrap();
            let tool_tmp_path = Path::new(&self.app_root_dir).join(format!("{}.zip", &tool_name));
            progress_tool(format!("Downloading {}...", tool_name.to_string()));
            if !tool_tmp_path.exists() {
                let file_downloaded = match File::create(&tool_tmp_path) {
                    Err(why) => panic!("couldn't create {}", why),
                    Ok(mut file) => {
                        let mut stream = res.bytes_stream();

                        while let Some(item) = stream.next().await {
                            let chunk = item
                                .or(Err(format!("Error while downloading file")))
                                .unwrap();
                            file.write(&chunk)
                                .or(Err(format!("Error while writing to file")))
                                .unwrap();
                        }
                        file
                    }
                };
                println!("{:?} downloaded", tool_tmp_path.as_path().as_os_str());
                unzip_file(&tool_tmp_path.to_str().unwrap(), &tool_dest_path_str).await.unwrap();
            }
            progress_tool(format!("{} downloaded!", &tool_name));
        }
    }

    pub async fn install(&mut self, progress_install: impl Fn(String)) {
        let client = Client::new();
        let app_dir = Path::new(&self.app_root_dir);
        self.installing = true;

        // Create directory
        if !&app_dir.exists() {
            progress_install("Creating directory...".to_string());
            std::fs::create_dir_all(&app_dir).unwrap();
        }

        // Copy binary
        match env::current_exe() {
            Ok(exe_path) => {
                progress_install("Copying Binary...".to_string());
                std::fs::copy(exe_path, app_dir.join(BINARY_NAME)).unwrap();
                if !cfg!(target_os = "windows") {
                    progress_install("Set Enviroment Variables...".to_string());
                    self.set_env_vars();
                }
            }
            Err(_) => println!("Error copying binary"),
        }

        // Install tools
        self.install_tools(client, &progress_install).await;

        // Create conf.json
        let conf_path = app_dir.join("conf.json");
        if !conf_path.exists() {
            progress_install("Creating Default Configurations...".to_string());
            let conf = Config::new();
            let conf_str = serde_json::to_string(&conf).unwrap();
            let mut file = File::create(&conf_path).unwrap();
            file.write_all(conf_str.as_bytes()).unwrap();
        }

        // Generate shortcuts
        if self.desktop_shortcut || self.menu_shortcut {
            progress_install("Generating shortcuts...".to_string());
            self.generate_shortcut(
                self.version.clone(),
                self.description.clone(),
                self.desktop_shortcut,
                self.menu_shortcut,
            ).await;
        }
        progress_install("Success Installed".to_string());
    }

    pub async fn generate_shortcut(
        &self,
        version: String,
        description: String,
        on_desktop: bool,
        on_startup_menu: bool,
    ) {
        let path = self.get_installed_path();

        generate_shortcut(path, version, description);
        todo!("Add \"from\" and \"to\" path");
    }
}

pub fn get_default_install_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    return Some(
        data_local_dir()
            .unwrap()
            .join(AUTHOR)
            .join(APP_NAME.replace(" ", "")),
    );

    // TODO: Add this path into enviroment variables

    // Another than not working on windows
    return Some(
        executable_dir()
            .unwrap()
            .join(AUTHOR)
            .join(APP_NAME.replace(" ", "")),
    );
}

pub fn get_install_path_tool() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    return Some(
        data_local_dir()
            .unwrap()
            .join(AUTHOR)
            .join(APP_NAME.replace(" ", "")),
    );

    // Another than not working on windows
    return Some(
        executable_dir()
            .unwrap()
            .join(AUTHOR)
            .join(APP_NAME.replace(" ", "")),
    );
}

mod test {
    use super::Installer;

    #[test]
    fn install_instance() {
        let mut installer = Installer::new("1.0.1".to_string());

        assert_eq!(installer.version, "1.0.1".to_string());
        assert!(!installer.is_installed());
        assert!(installer.desktop_shortcut);
        assert!(installer.menu_shortcut);

        installer.set_menu_shortcut(false);
        assert!(!installer.menu_shortcut);

        installer.set_desktop_shortcut(false);
        assert!(!installer.desktop_shortcut);
    }

    #[test]
    fn install_path_installation_default () {
    }

    #[test]
    fn install_path_installation() {
    }
}