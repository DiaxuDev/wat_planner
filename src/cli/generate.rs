#![allow(clippy::zero_prefixed_literal)]

use std::{fmt::Write as FWrite, fs::File, io::Write};

use chrono::{Datelike, NaiveDate};

use crate::{cache::read_or_save, data::ClassKind, error::Result, fetcher::Term};

#[derive(clap::Args, Debug)]
pub struct GenerateCommand {
    /// WAT group identifier
    pub group: String,
    /// Which date to generate the schedule for. If left empty will be detected automatically
    pub date: Option<NaiveDate>,
}

impl GenerateCommand {
    pub fn run(&self) -> Result<()> {
        let date = self
            .date
            .unwrap_or_else(|| chrono::Local::now().date_naive());
        let term = Term::from_month(date.month());

        let data = read_or_save(&self.group, term)?;
        let start = date.week(chrono::Weekday::Mon).first_day();

        let content: String = (0..5).fold(String::new(), |mut out, i| {
            let day = start + chrono::TimeDelta::days(i as i64);
            let name = WEEKDAYS[i];

            let _ = write!(
                out,
                r#"<div class="day"><div class="header"><span class="weekday">{name}</span><span class="date">{}</span></div>"#,
                day.format("%d.%m")
            );
            if let Some(classes) = data.classes.get(&day) {
                for (class, hour) in classes.iter().zip(HOURS) {
                    match class {
                        Some(class) => {
                            let details = data.legend.get(&class.code);
                            let name = details.map_or(&class.code, |x| &x.name);
                            let style = details.map(|x| format!(r#" style="--color: #{}""#, x.color.hex())).unwrap_or_default();
                            let kind = class.kind.as_ref().map_or("?", |x| match x {
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
                            });

                            let room = class.room.as_deref().unwrap_or("Brak");

                            let professor = match class.kind {
                                Some(ref kind) => match details {
                                    Some(details) => details.professors.get(kind).map(|professors| professors.iter().filter_map(|x| x.name.as_deref()).collect::<Vec<_>>().join(", ")),
                                    None => None,
                                },
                                None => None,
                            }.unwrap_or_else(|| "Brak".into());

                            let _ = write!(out, r#"<div class="slot"><div class="class"{style}><div class="header"><span class="hour">{}:{:0>2} - {}:{:0>2}</span><span class="kind">{kind}</span></div><span class="name">{name}</span><span class="room">{room}</span><span class="professor">{professor}</span></div></div>"#, hour.0 / 60, hour.0 % 60, hour.1 / 60, hour.1 % 60);
                        }
                        None => {
                            let _ = write!(out, r#"<div class="slot"></div>"#);
                        }
                    }
                }
            }
            let _ = write!(out, "</div>");

            out
        });

        let mut file = File::create("schedule.html")?;
        file.write_all(TEMPLATE.replace("{{generated}}", &content).as_bytes())?;
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
