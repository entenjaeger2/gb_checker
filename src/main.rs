use chrono::{
    prelude::{Date, Utc},
    Datelike,
};
use std::env;
#[cfg(feature = "dynamic-data")]
use std::fs::OpenOptions;
#[cfg(feature = "dynamic-data")]
use std::io::Read;
use std::ptr::null_mut as NULL;
use winapi::um::winuser;

mod datastructures;
use datastructures::Birthdate;
#[cfg(feature = "dynamic-data")]
use datastructures::DATA_FILE;
#[cfg(feature = "dynamic-data")]
mod datainput;

fn main() {
    let args: Vec<_> = env::args().collect();
    let today = Utc::today();

    if args.len() > 1 {
        if args[1] == "--test" {
            let test_date = Birthdate {
                name: "Test".to_string(),
                date_day: today.day(),
                date_month: today.month(),
                date_year: today.year(),
            };
            handle_birthday(today, test_date);
        }
    }

    #[cfg(not(feature = "dynamic-data"))]
    {
        let bytes = include_bytes!("../data/dates.json");
        let data = String::from_utf8_lossy(bytes);

        let entries: Vec<Birthdate> = serde_json::from_str(&data).unwrap();

        handle_birthdays(today, entries);
    }
    #[cfg(feature = "dynamic-data")]
    {
        let args: Vec<_> = env::args().collect();

        let mut file = OpenOptions::new().read(true).open(DATA_FILE).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let mut entries: Vec<Birthdate> = serde_json::from_str(&data).unwrap();

        if args.len() > 1 {
            if args[1] == "--input" {
                entries.push(datainput::input(today));
                datainput::store_data(entries);
            }
        } else {
            handle_birthdays(today, entries);
        }
    }
}

fn handle_birthdays(today: Date<Utc>, entries: Vec<Birthdate>) {
    for entry in entries {
        if check_date(today, entry.date_day, entry.date_month) {
            handle_birthday(today, entry);
        }
    }
}

fn handle_birthday(today: Date<Utc>, entry: Birthdate) {
    webbrowser::open(&gen_florida_man_string(entry.date_day, entry.date_month)).unwrap();
    popup(&entry.name, today.year() - entry.date_year);
}

fn check_date(today: Date<Utc>, date_d: u32, date_m: u32) -> bool {
    today.month() == date_m && today.day() == date_d
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
