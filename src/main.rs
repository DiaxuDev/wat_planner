use std::process::exit;

use clap::Parser;
use yansi::Paint;

use crate::cli::Cli;

mod cli;
mod data;
mod error;
mod fetcher;
mod parser;

fn main() {
    // let raw = std::fs::read_to_string("schedule.json")?;
    // let data: Data = serde_json::from_str(&raw)?;

    // let today = chrono::Local::now().date_naive();
    // let start = today.week(chrono::Weekday::Mon).first_day();
    // for i in 0..7 {
    //     let day = start + chrono::TimeDelta::days(i);
    //     let classes = data.classes.get(&day);
    //     println!("{day} -> {classes:#?}");
    // }

    match Cli::parse().run() {
        Ok(()) => exit(0),
        Err(why) => {
            eprintln!("{}", why.red());
            exit(1);
        }
    }
}
