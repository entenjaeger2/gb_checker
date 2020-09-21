use crate::datastructures::{Birthdate, DATA_FILE};
use chrono::{
    prelude::{Date, Utc},
    Datelike,
};
use regex::Regex;
use serde_json::json;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

pub fn input(today: Date<Utc>) -> Birthdate {
    let stdin = io::stdin();
    let re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();

    println!("Input a new name to be stored:");

    let mut in_lines = stdin.lock().lines();
    let name = in_lines.next().unwrap().unwrap();

    let mut date_day: u32;
    let mut date_month: u32;
    let mut date_year: i32;

    loop {
        println!("Input the corresponding date in the right format (YYYY-MM-DD):");

        let raw_date = in_lines.next().unwrap().unwrap();

        if re.is_match(&raw_date) {
            let groups = re.captures(&raw_date).unwrap();
            date_year = i32::from_str_radix(groups.get(1).map_or("", |m| m.as_str()), 10).unwrap();
            date_month = u32::from_str_radix(groups.get(2).map_or("", |m| m.as_str()), 10).unwrap();
            date_day = u32::from_str_radix(groups.get(3).map_or("", |m| m.as_str()), 10).unwrap();

            if 1900 <= date_year
                && today.year() >= date_year
                && 1 <= date_month
                && 12 >= date_month
                && 1 <= date_day
                && 31 >= date_day
            {
                break;
            }
        }
    }

    Birthdate {
        name,
        date_year,
        date_month,
        date_day,
    }
}

pub fn store_data(date: Vec<Birthdate>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(DATA_FILE)
        .unwrap();
    let json = serde_json::to_string(&json!(date)).unwrap();

    if let Err(e) = writeln!(file, "{}", json) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
