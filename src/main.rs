extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate winapi;

use chrono::prelude::*;

use std::ptr::null_mut as NULL;
use winapi::um::winuser;
mod datastructs;
use datastructs::{Birthdate, CURRENT_YEAR};

fn main() {
	let bytes = include_bytes!("../data/dates.json");
	let data =  String::from_utf8_lossy(bytes);

	let entries: Vec<Birthdate> = serde_json::from_str(&data).unwrap();

	for entry in entries {
		if check_date(entry.date_day, entry.date_month) {
			popup(&entry.name, CURRENT_YEAR - entry.date_year);
		}
	}
}

fn check_date(date_d: u32, date_m: u32) -> bool {
	Utc::today().naive_utc() == NaiveDate::from_ymd(CURRENT_YEAR, date_m, date_d)
}

fn popup(name: &str, age: i32) {
	let l_msg: Vec<u16> = (name.to_owned()
		+ " hat heute Geburtstag!\nEs ist der "
		+ &(age.to_string())
		+ ". Geburtstag\0")
		.encode_utf16()
		.collect();
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
