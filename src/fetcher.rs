use chrono::Datelike;

pub fn fetch_schedule(term: Term, group: &str) -> reqwest::Result<String> {
    reqwest::blocking::get(format!(
        "https://wel.wat.edu.pl/planyzajec/{term}/{group}.htm"
    ))?
    .error_for_status()?
    .text()
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum Term {
    Winter,
    Summer,
}

impl Term {
    pub const fn value(self) -> &'static str {
        match self {
            Self::Winter => "zima",
            Self::Summer => "lato",
        }
    }

    pub fn detect() -> Self {
        match chrono::Local::now().month() {
            1 | 2 | 7..=12 => Self::Winter,
            3..=6 => Self::Summer,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value())
    }
}
