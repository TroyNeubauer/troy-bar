mod util;

fn main() {
	println!("Hello, world!");
	util::set_window_title(b"Rust is great 2\0");
	println!("Set window title");

}
