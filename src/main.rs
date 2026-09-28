use std::{
    fs::File,
    io::{Read, Write},
};

use chrono::Days;

use crate::{
    fetcher::{Term::Winter, fetch_schedule},
    parser::{Data, parse},
};

mod fetcher;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let raw = fetch_schedule(Winter, "WEL26TX8S0")?;
    // let data = parse(&raw);

    // let mut file = File::create("schedule.json")?;
    // serde_json::to_writer(&file, &data)?;
    // file.flush()?;

    let raw = std::fs::read_to_string("schedule.json")?;
    let data: Data = serde_json::from_str(&raw)?;

    let today = chrono::Local::now().date_naive();
    let start = today.week(chrono::Weekday::Mon).first_day();
    for i in 0..7 {
        let day = start + chrono::TimeDelta::days(i);
        let classes = data.classes.get(&day);
        println!("{day} -> {classes:#?}");
    }

    Ok(())
}
