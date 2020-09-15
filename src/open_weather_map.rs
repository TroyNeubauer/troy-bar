
use chrono::{DateTime, TimeZone, Utc, Local, Duration};


pub struct Data {
	pub high: f32, pub low: f32,
	pub now: f32,
	pub sunrise: DateTime<Local>, pub sunset: DateTime<Local>,
	pub humidity: f32,
	pub today_precipitation_percent: f32,
	pub hour_precipitation_percent: f32,
	last_update: chrono::DateTime<Utc>,
}

pub struct Arguments {
	pub latitude: f64,
	pub longitude: f64,
	pub api_key: String,
}

impl Data {
	pub fn new() -> Data {
	
		Data {
			high: 100.0,
			low: 0.0,
			now: 0.0,
			sunrise: Local::now(),
			sunset: Local::now(),
			humidity: 0.0,
			today_precipitation_percent: 0.0,
			hour_precipitation_percent: 0.0,
			last_update: Utc::now() - Duration::days(1),//Force an update on startup
		}
	}
}


//static mut 

pub fn update(data: &mut Data, args: &Arguments) {

	let weather_update_rate = Duration::seconds(90);//They allow 1000 requests per day on the free tier: = 86.4 seconds between request to avoid getting rate limited
	let difference = Utc::now().signed_duration_since(data.last_update);
	if difference > weather_update_rate {
		data.last_update = Utc::now();
		request_data(data, args);
		println!("Weather data updated");
	}
}

fn kelvin_to_f(k: f64) -> f64 {
	(k - 273.15) * 9.0/5.0 + 32.0
}

fn request_data(data: &mut Data, args: &Arguments) {
	let req_url = format!("https://api.openweathermap.org/data/2.5/onecall?lat={}&lon={}&appid={}", args.latitude, args.longitude, args.api_key);
	let response = reqwest::blocking::get(req_url.as_str()).unwrap().text().unwrap();

	let result = json::parse(response.as_str()).unwrap();
	if result["message"].as_str().is_some() {
		println!("Failed to request weather data!");
		println!("Code {}, message {}", result["cod"].as_u32().unwrap(), result["message"].as_str().unwrap());
		data.high = 0.0;
		data.low = 0.0;
		data.humidity = 0.0;
	
		data.now = 0.0;
	
		data.sunrise = Local.timestamp(0, 0);
		data.sunset = Local.timestamp(0, 0);
		data.today_precipitation_percent = 0.0;
		data.hour_precipitation_percent = 0.0;
	} else {
		let current = &result["current"];
		let hour = &result["hourly"][0];
		let today = &result["daily"][0];
	
		data.high = kelvin_to_f(today["temp"]["max"].as_f64().unwrap()) as f32;
		data.low = kelvin_to_f(today["temp"]["min"].as_f64().unwrap()) as f32;
		data.humidity = current["humidity"].as_f64().unwrap() as f32;
	
		data.now = kelvin_to_f(current["temp"].as_f64().unwrap()) as f32;
	
		data.sunrise = Local.timestamp(current["sunrise"].as_i64().unwrap(), 0);
		data.sunset = Local.timestamp(current["sunset"].as_i64().unwrap(), 0);
		data.today_precipitation_percent = today["pop"].as_f64().unwrap() as f32;
		data.hour_precipitation_percent = hour["pop"].as_f64().unwrap() as f32;
	
	}	
}

