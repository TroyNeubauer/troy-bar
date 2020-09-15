mod util;

mod open_weather_map;

use chrono::{Local, Utc};

use std::{thread, time, env};


fn main() {
	let str_args: Vec<String> = env::args().collect();
	if str_args.len() != 4 {
		println!("Expected 3 args but got {}", str_args.len());
		panic!("USAGE: troybar latitude longitude openweathermap.org-key");
	}
	let args = open_weather_map::Arguments {
		latitude:  str_args[1].parse::<f64>().unwrap(),
		longitude: str_args[2].parse::<f64>().unwrap(),
		api_key:  str_args[3].clone(),
	};


	let mut s = String::new();
	let mut data: open_weather_map::Data = open_weather_map::Data::new();

	let mut next_sec = (Utc::now().timestamp_millis() / 1000) * 1000 + 1000;//TODO figure out how to use the time library properly so we can avoid nasty math
	loop {
		s.clear();
		open_weather_map::update(&mut data, &args);

		bar_weather(&mut s, &mut data);
		separator(&mut s);
		bar_time(&mut s);

		s.push_str("\0");
		util::set_window_title(s.as_bytes());
		let sleep_time = next_sec - Utc::now().timestamp_millis();
		if sleep_time > 0 {
			next_sec += 1000;
			thread::sleep(time::Duration::from_millis(sleep_time as u64));
		} else {
			next_sec = (Utc::now().timestamp_millis() / 1000) * 1000 + 1000
		}
	}
}

#[inline]
fn separator(s: &mut String) {
	s.push_str(" | ");
}

fn bar_time(s: &mut String) {
	let now = Local::now();
	let dt_string = now.format("%Y/%m/%d %H:%M:%S");
	s.push_str(format!("{}", dt_string).as_str());
}

fn bar_weather(s: &mut String, data: &mut open_weather_map::Data) {
	s.push_str(format!("{:.1}°F @{:.0}% : {:.0}-{:.0}°F {}-{} T{:.0}% H{:.0}%", data.now, data.humidity, data.low, data.high, data.sunrise.format("%H:%M"), data.sunset.format("%H:%M"), data.today_precipitation_percent, data.hour_precipitation_percent).as_str());
}
