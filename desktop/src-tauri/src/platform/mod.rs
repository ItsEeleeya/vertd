#[cfg(target_os = "macos")]
#[path = "macos.rs"]
mod platform_impl;

// #[cfg(target_os = "windows")]
// #[path = "windows.rs"]
// mod platform_impl;

pub use platform_impl::*;
