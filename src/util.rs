
extern crate x11;
use std::ptr;
use std::ffi::CStr;


#[link(name = "X11")]
extern {}

pub fn set_window_title(bytes: &[u8]) {
	unsafe {
		let display = x11::xlib::XOpenDisplay(ptr::null());
		let screen = x11::xlib::XDefaultScreen(display);
		let root = x11::xlib::XRootWindow(display, screen);
		x11::xlib::XStoreName(display, root, CStr::from_bytes_with_nul(bytes).unwrap().as_ptr());

		x11::xlib::XCloseDisplay(display);
	}

}
