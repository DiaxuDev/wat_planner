use std::collections::HashMap;

use scraper::{ElementRef, Selector, selectable::Selectable};

use crate::data::{Class, ClassKind, Color, Data, Professor, Subject};

pub fn parse(raw: &str) -> Result<Data> {
    let html = scraper::Html::parse_document(raw);

    let table_selector = Selector::parse("body table tbody").map_err(|_| Error::InvalidSelector)?;
    let table = html
        .select(&table_selector)
        .next()
        .ok_or(Error::NoElement)?;

    let header_selector = Selector::parse("body>b:first-child>font>p:nth-child(3)>b>b>font")
        .map_err(|_| Error::InvalidSelector)?;
    let (lower, upper) = html
        .select(&header_selector)
        .next()
        .ok_or(Error::NoElement)?
        .text()
        .next()
        .ok_or(Error::NoText)?
        .trim_matches(|c: char| !c.is_ascii_digit() && c != '/')
        .split_once('/')
        .ok_or(Error::UnexpectedFormat)?;

    let year = (lower.parse()?, upper.parse()?);
    let legend = parse_legend(&table)?;
    let classes = parse_classes(&table, year)?;

    Ok(Data {
        year,
        legend,
        classes,
    })
}

fn parse_legend(table: &ElementRef) -> Result<HashMap<String, Subject>> {
    let rows_selector = Selector::parse("tr").map_err(|_| Error::InvalidSelector)?;
    let last_selector = Selector::parse("td:last-child").map_err(|_| Error::InvalidSelector)?;
    let second_last_selector =
        Selector::parse("td:nth-last-child(2)").map_err(|_| Error::InvalidSelector)?;

    let mut subjects = HashMap::<String, Subject>::new();

    let mut curr_span: Option<String> = None;
    let mut curr_header: Option<(String, String, Color)> = None;

    let mut professors = HashMap::<ClassKind, Vec<Professor>>::new();

    for element in table.select(&rows_selector).skip(1) {
        if let Some(name) = curr_span {
            let details = element
                .select(&last_selector)
                .next()
                .ok_or(Error::NoElement)?
                .text()
                .next()
                .ok_or(Error::NoText)?;

            let (kind, hours) = details.split_once(' ').ok_or(Error::UnexpectedFormat)?;

            let kind = ClassKind::from(kind.to_lowercase().as_str());
            let hours: u8 = hours.parse()?;

            let professor = Professor {
                name: Some(name),
                hours,
            };

            if let Some(current) = professors.get_mut(&kind) {
                current.push(professor);
            } else {
                professors.insert(kind, vec![professor]);
            }

            curr_span = None;
        } else {
            let first = element
                .select(&second_last_selector)
                .next()
                .ok_or(Error::NoElement)?;
            let second = element
                .select(&last_selector)
                .next()
                .ok_or(Error::NoElement)?;

            let header_color = first
                .attr("style")
                .map(|style| {
                    style.split(';').find_map(|x| {
                        x.split_once(':').and_then(|(key, value)| match key {
                            "background-color" => value.parse().ok(),
                            _ => None,
                        })
                    })
                })
                .unwrap_or_default();

            if let Some(color) = header_color {
                if let Some((abbreviation, name, color)) = curr_header {
                    subjects.insert(
                        abbreviation,
                        Subject {
                            name,
                            color,
                            professors,
                        },
                    );

                    professors = HashMap::new();
                }

                let abbreviation = first.text().next().ok_or(Error::NoText)?.to_owned();
                let name = second.text().next().ok_or(Error::NoText)?.to_owned();

                curr_header = Some((abbreviation, name, color));
            } else {
                let name_el = element
                    .select(&last_selector)
                    .next()
                    .ok_or(Error::NoElement)?;
                let name = name_el
                    .text()
                    .next()
                    .filter(|x| !x.trim().is_empty())
                    .map(str::to_owned);

                if name_el.attr("rowspan").is_some() {
                    curr_span.clone_from(&name);
                }

                let details = element
                    .select(&second_last_selector)
                    .next()
                    .ok_or(Error::NoElement)?
                    .text()
                    .next()
                    .ok_or(Error::NoText)?;

                let (kind, hours) = details.split_once(' ').ok_or(Error::UnexpectedFormat)?;

                let kind = ClassKind::from(kind.to_lowercase().as_str());
                let hours: u8 = hours.parse()?;

                let professor = Professor { name, hours };

                if let Some(current) = professors.get_mut(&kind) {
                    current.push(professor);
                } else {
                    professors.insert(kind, vec![professor]);
                }
            }
        }
    }

    if let Some((abbreviation, name, color)) = curr_header {
        subjects.insert(
            abbreviation,
            Subject {
                name,
                color,
                professors,
            },
        );
    }

    Ok(subjects)
}

fn parse_classes(
    table: &ElementRef,
    year: (u16, u16),
) -> Result<HashMap<chrono::NaiveDate, [Option<Class>; 7]>> {
    let header_selector =
        Selector::parse("tr:first-child>td").map_err(|_| Error::InvalidSelector)?;

    let total_columns = table
        .select(&header_selector)
        .skip(2)
        .take(5)
        .map(|x| {
            x.attr("colspan")
                .and_then(|v| v.parse::<u8>().ok())
                .unwrap_or_default()
        })
        .sum::<u8>();

    let mut result = HashMap::new();
    let mut offsets = [(0, 0); 55];

    for col in 0..total_columns {
        for weekday in 0..7 {
            let day_header_selector = Selector::parse(&format!(
                "tr:nth-child({})>td:nth-child({})",
                2 + weekday * 8,
                3 + col
            ))
            .map_err(|_| Error::InvalidSelector)?;

            let day_header = table
                .select(&day_header_selector)
                .next()
                .ok_or(Error::NoElement)?;
            if matches!(day_header.attr("background"), Some("outofrange.gif")) {
                continue;
            }

            let (day, month) = day_header
                .text()
                .next()
                .ok_or(Error::NoText)?
                .split_once(' ')
                .ok_or(Error::UnexpectedFormat)?;
            let month = month_from_roman(month)?;
            let date = chrono::NaiveDate::from_ymd_opt(
                if month > 9 {
                    year.0 as i32
                } else {
                    year.1 as i32
                },
                month,
                day.parse::<u32>()?,
            )
            .ok_or(Error::UnexpectedFormat)?;

            let mut classes: [Option<Class>; 7] = Default::default();
            let mut span: (Option<Class>, usize) = Default::default();
            for (i, item) in classes.iter_mut().enumerate() {
                if span.1 > 0 {
                    span.1 -= 1;

                    if span.1 == 0 {
                        *item = span.0;
                        span = (None, 0);
                    } else {
                        item.clone_from(&span.0);
                    }

                    continue;
                }

                let row = weekday * 8 + i;

                let class_selector = Selector::parse(&format!(
                    "tr:nth-child({})>td:nth-child({})",
                    row + 3,
                    2 + col - offsets[row].0 + offsets[row].1
                ))
                .map_err(|_| Error::InvalidSelector)?;

                if offsets[row].1 > 0 {
                    offsets[row].1 -= 1;
                }

                let class = table
                    .select(&class_selector)
                    .next()
                    .ok_or(Error::NoElement)?;

                if class.attr("bgcolor").is_some() {
                    let mut values = class.text();
                    let code = values.next().ok_or(Error::NoText)?.to_owned();
                    let kind = values
                        .next()
                        .map(|x| ClassKind::from(x.to_lowercase().as_str()));
                    let rest = values.collect::<String>();

                    let room = if rest.trim().is_empty() {
                        None
                    } else {
                        Some(rest)
                    };

                    *item = Some(Class { code, kind, room });

                    if let Some(cols) = class.attr("colspan").and_then(|v| v.parse::<u8>().ok()) {
                        offsets[row].0 += cols - 1;
                        offsets[row].1 = cols - 2;
                    }

                    if let Some(rows) = class.attr("rowspan").and_then(|v| v.parse::<usize>().ok())
                    {
                        span = (item.clone(), rows - 1);
                        for offset in offsets.iter_mut().skip(row + 1).take(rows) {
                            offset.0 += 1;
                        }
                    }
                }
            }

            if classes.iter().any(Option::is_some) {
                result.insert(date, classes);
            }
        }
    }

    Ok(result)
}

fn month_from_roman(value: &str) -> Result<u32> {
    match value {
        "I" => Ok(1),
        "II" => Ok(2),
        "III" => Ok(3),
        "IV" => Ok(4),
        "V" => Ok(5),
        "VI" => Ok(6),
        "VII" => Ok(7),
        "VIII" => Ok(8),
        "IX" => Ok(9),
        "X" => Ok(10),
        "XI" => Ok(11),
        "XII" => Ok(12),
        _ => Err(Error::UnexpectedFormat),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid selector")]
    InvalidSelector,
    #[error("element not found")]
    NoElement,
    #[error("no text found")]
    NoText,
    #[error("text was in an unexpected format")]
    UnexpectedFormat,
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

type Result<T> = std::result::Result<T, Error>;
