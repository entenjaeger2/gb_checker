extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate webbrowser;
extern crate winapi;

use chrono::prelude::*;
use std::ptr::null_mut as NULL;
use winapi::um::winuser;

mod datastructs;
use datastructs::Birthdate;

fn main() {
    let bytes = include_bytes!("../data/dates.json");
    let data = String::from_utf8_lossy(bytes);

    let entries: Vec<Birthdate> = serde_json::from_str(&data).unwrap();

    for entry in entries {
        if check_date(entry.date_day, entry.date_month) {
            webbrowser::open(&gen_florida_man_string(entry.date_day, entry.date_month)).unwrap();
            popup(&entry.name, Utc::today().year() - entry.date_year);
        }
    }
}

fn check_date(date_d: u32, date_m: u32) -> bool {
    Utc::today().naive_utc() == NaiveDate::from_ymd(Utc::today().year(), date_m, date_d)
}

fn month_string(date_m: u32) -> String {
    match date_m {
        1 => "january",
        2 => "february",
        3 => "march",
        4 => "april",
        5 => "may",
        6 => "june",
        7 => "july",
        8 => "august",
        9 => "september",
        10 => "october",
        11 => "november",
        12 => "december",
        _ => "none",
    }
    .to_string()
}

fn gen_florida_man_string(date_d: u32, date_m: u32) -> String {
    let base = "https://duckduckgo.com/?q=florida+man+".to_string();
    let month = month_string(date_m);
    let day = date_d.to_string();
    format!("{}{}+{}", base, month, day)
}

fn popup(name: &str, age: i32) {
    let l_msg: Vec<u16> = (name.to_owned()
        + " hat heute Geburtstag!\nEs ist der "
        + &(age.to_string())
        + ". Geburtstag\0")
        .encode_utf16()
        .collect();
    let l_title: Vec<u16> = "Geburtstag!\0".encode_utf16().collect();

    unsafe {
        winuser::MessageBoxW(
            NULL(),
            l_msg.as_ptr(),
            l_title.as_ptr(),
            winuser::MB_OK | winuser::MB_ICONINFORMATION,
        );
    }
}
