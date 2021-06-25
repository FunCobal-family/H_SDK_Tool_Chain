#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub mod another;
#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub use self::another::*;
