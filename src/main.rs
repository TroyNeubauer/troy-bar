mod util;

use std::{thread, time};

fn main() {
	let mut s = String::new();
	loop {
		s.clear();
		
		bar_time(& mut s);

		s.push_str("\0");
		util::set_window_title(s.as_bytes());
		thread::sleep(time::Duration::from_millis(1000));
	}
}

extern crate chrono;

use chrono::Local;


fn bar_time(s: &mut String)
{

	let now = Local::now();
	let dt_string = now.format("%Y/%m/%d %H:%M:%S");
	s.push_str(format!("{}", dt_string).as_str());
}
