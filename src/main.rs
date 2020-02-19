extern crate chrono;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate winapi;

use chrono::prelude::*;
use std::env;
use std::io::Read;
use std::ptr::null_mut as NULL;
use winapi::um::winuser;
mod datainput;
use datainput::datastructs::{Birthdate, CURRENT_YEAR, DATA_FILE};
use std::fs::OpenOptions;

fn main() {
	let args: Vec<_> = env::args().collect();

	let mut file = OpenOptions::new().read(true).open(DATA_FILE).unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();

	let mut entries: Vec<Birthdate> = serde_json::from_str(&data).unwrap();

	if args.len() > 1 {
		if args[1] == "--input" {
			entries.push(datainput::input());
			datainput::store_data(entries);
		}
	} else {
		for entry in entries {
			if check_date(entry.date_day, entry.date_month) {
				popup(&entry.name, CURRENT_YEAR - entry.date_year);
			}
		}
	}
}

fn check_date(date_d: u32, date_m: u32) -> bool {
	let today = Utc::today();
	today.month() == date_m && today.day() == date_d
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
