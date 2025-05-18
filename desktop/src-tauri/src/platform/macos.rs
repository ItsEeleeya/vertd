use std::ffi::c_void;

use swift_rs::{Double, swift};

swift!(pub fn apply_swiftui_window_chrome(nswindow_ptr: *mut c_void, corner_radius: Double));
