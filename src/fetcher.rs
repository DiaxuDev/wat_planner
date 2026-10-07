pub fn fetch(term: Term, group: &str) -> reqwest::Result<String> {
    let branch = group[..3].to_lowercase();

    reqwest::blocking::get(format!(
        "https://{branch}.wat.edu.pl/planyzajec/{term}/{group}.htm"
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

    pub fn from_month(month: u32) -> Self {
        match month {
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
