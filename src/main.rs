extern crate winapi;
extern crate chrono;
extern crate rustc_serialize;

use std::ptr::null_mut as NULL;
use winapi::um::winuser;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use chrono::prelude::*;

const CURRENT_YEAR: i32 = 2019;

pub struct DateDay {
	day: u32,
	month: u32
}

pub struct Year {
	year: i32
}

fn main() {
	let mut file = File::open("data/dates.json").unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	let json = Json::from_str(&data).unwrap();
	
	let entries = match json {
		Json::Array(entries) => Some(entries),
		_ => None
	}.unwrap();

	for entry in entries {
		let (n, d, y) = deserial_entry(entry);
		if check_date(&d) {
			popup(n, CURRENT_YEAR-y.year);
		}
	}
}

fn deserial_entry(entry: Json) -> (String, DateDay, Year) {
	match entry {
		Json::Object(_) => {
			let name = entry.find("name").unwrap().as_string().unwrap().to_string();
			let (d, y) = date_of_string(entry.find("date").unwrap().as_string().unwrap().to_string());
			(name, d, y)
		}
		_ => {
			 let (d, y) = date_of_string("0.0.0".to_string());
			("".to_string(), d, y)
		}
	}
}

fn date_of_string(dstring: String) -> (DateDay, Year) {
	let mut iters = dstring.split(".");
	let day = u32::from_str_radix(iters.next().unwrap(), 10).unwrap();
	let month = u32::from_str_radix(iters.next().unwrap(), 10).unwrap();
	let year = i32::from_str_radix(iters.next().unwrap(), 10).unwrap();

	(DateDay{day, month}, Year{year})
}

fn check_date(date: &DateDay) -> bool {
	Utc::today().naive_utc() == NaiveDate::from_ymd(CURRENT_YEAR, date.month, date.day)
}

fn popup(name: String, age:i32) {
	let l_msg: Vec<u16> = (name + " hat heute Geburtstag!\nEs ist der " + &(age.to_string()) + ". Geburtstag\0").encode_utf16().collect();
	let l_title: Vec<u16> = "Geburtstag\0".encode_utf16().collect();

	unsafe {
		winuser::MessageBoxW(
			NULL(),
			l_msg.as_ptr(),
			l_title.as_ptr(),
			winuser::MB_OK | winuser::MB_ICONINFORMATION,
		);
	}
}
