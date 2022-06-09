
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::generate_shortcut;

#[cfg(target_os = "macos")]
pub use macos::generate_shortcut;

#[cfg(target_os = "windows")]
pub use windows::generate_shortcut;
