use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Version,
};

pub fn fetch(term: Term, group: &str) -> reqwest::Result<String> {
    let branch = group[..3].to_lowercase();

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .default_headers(construct_headers())
        .build()?;

    client
        .get(format!(
            "https://{branch}.wat.edu.pl/planyzajec/{term}/{group}.htm"
        ))
        .version(Version::HTTP_2)
        .send()?
        .error_for_status()?
        .text()
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::default();
    headers.extend([
        (
            header::USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64; rv:156.0) Gecko/20100101 Firefox/156.0",
            ),
        ),
        (
            header::ACCEPT,
            HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            ),
        ),
        (
            header::ACCEPT_LANGUAGE,
            HeaderValue::from_static("pl,en-US;q=0.9"),
        ),
    ]);
    headers
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
