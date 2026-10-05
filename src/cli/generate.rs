#![allow(clippy::zero_prefixed_literal)]

use std::{fs::File, io::Write};

use chrono::{Datelike, NaiveDate};
use serde::Serialize;
use tera::{Tera, context};

use crate::{cache::read_or_save, data::ClassKind, error::Result, fetcher::Term};

#[derive(clap::Args, Debug)]
pub struct GenerateCommand {
    /// WAT group identifier
    pub group: String,
    /// Which date to generate the schedule for. If left empty will be detected automatically
    pub date: Option<NaiveDate>,
}

#[derive(Serialize)]
struct TeraDay<'a> {
    weekday: &'static str,
    date: String,
    classes: [Option<TeraClass<'a>>; 7],
}

#[derive(Serialize)]
struct TeraClass<'a> {
    hour: String,
    name: &'a str,
    kind: &'a str,
    color: Option<String>,
    professors: Vec<&'a str>,
    info: &'a Vec<String>,
}

impl GenerateCommand {
    pub fn run(&self) -> Result<()> {
        let date = self
            .date
            .unwrap_or_else(|| chrono::Local::now().date_naive());

        let data = read_or_save(&self.group, Term::from_month(date.month()))?;
        let start = date.week(chrono::Weekday::Mon).first_day();

        let days: Vec<TeraDay> = WEEKDAYS
            .iter()
            .enumerate()
            .map(|(i, weekday)| {
                let day = start + chrono::TimeDelta::days(i as i64);
                let mut classes: [Option<TeraClass>; 7] = Default::default();

                if let Some(values) = data.classes.get(&day) {
                    for i in 0..classes.len() {
                        if let Some(ref class) = values[i] {
                            let hour = HOURS[i];
                            let hour = format!(
                                "{}:{:0>2} - {}:{:0>2}",
                                hour.0 / 60,
                                hour.0 % 60,
                                hour.1 / 60,
                                hour.1 % 60
                            );

                            let (name, color, professors) = match data.legend.get(&class.code) {
                                Some(details) => {
                                    let professors = details
                                        .professors
                                        .get(&class.kind)
                                        .map(|professors| {
                                            professors
                                                .iter()
                                                .filter_map(|x| x.name.as_deref())
                                                .collect()
                                        })
                                        .unwrap_or_default();

                                    (&details.name, Some(details.color.hex()), professors)
                                }
                                None => (&class.code, None, Vec::default()),
                            };

                            let kind = match &class.kind {
                                ClassKind::None => "?",
                                ClassKind::Lecture => "wykład",
                                ClassKind::Practice => "ćwiczenia",
                                ClassKind::Lab => "laboratorium",
                                ClassKind::Seminar => "seminaria",
                                ClassKind::Project => "projekt",
                                ClassKind::Exam => "egzamin",
                                ClassKind::MakeUpExam => "egzamin poprawkowy",
                                ClassKind::Pass => "zaliczenie",
                                ClassKind::MakeUpPass => "zaliczenie poprawkowe",
                                ClassKind::Retake => "powtarzany przedmiot",
                                ClassKind::Unknown(inner) => inner,
                            };

                            classes[i] = Some(TeraClass {
                                hour,
                                name,
                                kind,
                                color,
                                professors,
                                info: &class.info,
                            });
                        }
                    }
                }

                TeraDay {
                    date: day.format("%d.%m").to_string(),
                    weekday,
                    classes,
                }
            })
            .collect();

        let mut file = File::create("schedule.html")?;

        Tera::default().render_str_to(TEMPLATE, &context! { days => &days }, false, &file)?;
        file.flush()?;

        Ok(())
    }
}

macro_rules! hour {
    ($h:expr;$m:expr) => {
        $h * 60 + $m
    };
    ($h1:expr;$m1:expr => $h2:expr;$m2:expr) => {
        (hour!($h1;$m1), hour!($h2;$m2))
    }
}

const TEMPLATE: &str = include_str!("../../include/template.html");
const WEEKDAYS: [&str; 5] = ["Poniedziałek", "Wtorek", "Środa", "Czwartek", "Piątek"];
const HOURS: [(u16, u16); 7] = [
    hour!(8;00 => 9;35),
    hour!(9;50 => 11;25),
    hour!(11;40 => 13;15),
    hour!(13;30 => 15;05),
    hour!(16;00 => 17;35),
    hour!(17;50 => 19;25),
    hour!(19;40 => 21;15),
];
