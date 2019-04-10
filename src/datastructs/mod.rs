use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Birthdate {
    pub name: String,
    pub date_day: u32,
    pub date_month: u32,
    pub date_year: i32,
}

pub const CURRENT_YEAR: i32 = 2019;
