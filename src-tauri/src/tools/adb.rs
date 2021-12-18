use std::process::Command;

use super::Tool;

pub struct Adb {
    pub device: String,
    pub port: u16,
    pub adb_path: String,
}

impl Tool for Adb {

    fn name(&self) -> &str { "adb" }

    fn description(&self) -> &str { "Android Debug Bridge" }

}

impl Adb {

    pub fn new(device: String, port: u16, adb_path: String) -> Adb {
        Adb {
            device,
            port,
            adb_path,
        }
    }

    pub fn get_devices(&self) -> Vec<String> {
        let mut cmd = Command::new(&self.adb_path);
        cmd.arg("devices");
        let output = cmd.output().expect("Failed to execute adb");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();
        for line in stdout.lines() {
            if line.contains("device") {
                let device = line.split("\t").collect::<Vec<&str>>()[0].to_string();
                devices.push(device);
            }
        }
        devices
    }

    pub fn get_applications(&self) -> Vec<String> {
        let mut cmd = Command::new(&self.adb_path);
        cmd.arg("shell");
        cmd.arg("pm");
        cmd.arg("list");
        cmd.arg("packages");
        let output = cmd.output().expect("Failed to execute adb");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut applications = Vec::new();
        for line in stdout.lines() {
            if line.contains("package") {
                let application = line.split("\t").collect::<Vec<&str>>()[1].to_string();
                applications.push(application);
            }
        }
        applications
    }

    pub fn get_logcat_by_package_and_tag_verbosity(&self, package: &str, tag: &str, verbosity: &str) -> String {
        let mut cmd = Command::new(&self.adb_path);
        cmd.arg("logcat");
        cmd.arg("-s");
        cmd.arg(format!("{}/{}", package, tag));
        cmd.arg(verbosity);
        let output = cmd.output().expect("Failed to execute adb");
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.to_string()
    }

    pub fn install_apk(&self, apk_path: &str) -> String {
        let mut cmd = Command::new(&self.adb_path);
        cmd.arg("install");
        cmd.arg(apk_path);
        let output = cmd.output().expect("Failed to execute adb");
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.to_string()
    }

    pub fn uninstall_apk(&self, package: &str) -> String {
        let mut cmd = Command::new(&self.adb_path);
        cmd.arg("uninstall");
        cmd.arg(package);
        let output = cmd.output().expect("Failed to execute adb");
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.to_string()
    }

    pub fn launch_apk(&self, package: &str) -> String {
        let mut cmd = Command::new(&self.adb_path);
        cmd.arg("shell");
        cmd.arg("am");
        cmd.arg("start");
        cmd.arg(format!("-n {}/{}", package, package));
        let output = cmd.output().expect("Failed to execute adb");
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.to_string()
    }

    pub fn stop_apk(&self, package: &str) -> String {
        let mut cmd = Command::new(&self.adb_path);
        cmd.arg("shell");
        cmd.arg("am");
        cmd.arg("force-stop");
        cmd.arg(package);
        let output = cmd.output().expect("Failed to execute adb");
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.to_string()
    }

    pub fn clear_logcat(&self) -> String {
        let mut cmd = Command::new(&self.adb_path);
        cmd.arg("logcat");
        cmd.arg("-c");
        let output = cmd.output().expect("Failed to execute adb");
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.to_string()
    }

    pub fn open_with_url(&self, url: &str) -> String {
        let mut cmd = Command::new(&self.adb_path);
        cmd.arg("shell");
        cmd.arg("am");
        cmd.arg("start");
        cmd.arg("-a");
        cmd.arg("android.intent.action.VIEW");
        cmd.arg("-d");
        cmd.arg(url);
        let output = cmd.output().expect("Failed to execute adb");
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.to_string()
    }
}
