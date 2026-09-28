pub fn fetch_schedule(term: Term, group: &str) -> reqwest::Result<String> {
    reqwest::blocking::get(format!(
        "https://wel.wat.edu.pl/planyzajec/{group}/{term}.htm"
    ))?
    .error_for_status()?
    .text()
}

#[derive(Clone, Copy)]
pub enum Term {
    Winter,
    Summer,
}

impl Term {
    pub fn value(self) -> &'static str {
        match self {
            Term::Winter => "zima",
            Term::Summer => "lato",
        }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value())
    }
}
