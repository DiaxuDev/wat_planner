use std::collections::HashMap;

use scraper::{ElementRef, Selector, selectable::Selectable};

use crate::data::{Class, ClassKind, Color, Data, Professor, Subject};

pub fn parse(raw: &str) -> Data {
    let html = scraper::Html::parse_document(raw);

    let table_selector = Selector::parse("body table tbody").unwrap();
    let table = html.select(&table_selector).next().unwrap();

    let legend = parse_legend(&table);
    let classes = parse_classes(&table);

    Data { legend, classes }
}

fn parse_legend(table: &ElementRef) -> HashMap<String, Subject> {
    let rows_selector = Selector::parse("tr").unwrap();
    let last_selector = Selector::parse("td:last-child").unwrap();
    let second_last_selector = Selector::parse("td:nth-last-child(2)").unwrap();

    let mut subjects = HashMap::<String, Subject>::new();

    let mut curr_span: Option<String> = None;
    let mut curr_header: Option<(String, String, Color)> = None;

    let mut professors = HashMap::<ClassKind, Vec<Professor>>::new();

    for element in table.select(&rows_selector).skip(1) {
        if let Some(name) = curr_span {
            let details = element
                .select(&last_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap();

            let (kind, hours) = details.split_once(' ').unwrap();

            let kind = ClassKind::from(kind.to_lowercase().as_str());
            let hours: u8 = hours.parse().unwrap();

            let professor = Professor {
                name: (!name.trim().is_empty()).then_some(name),
                hours,
            };

            if let Some(current) = professors.get_mut(&kind) {
                current.push(professor);
            } else {
                professors.insert(kind, vec![professor]);
            }

            curr_span = None;
        } else {
            let first = element.select(&second_last_selector).next().unwrap();
            let second = element.select(&last_selector).next().unwrap();

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

                let abbreviation = first.text().next().unwrap().to_owned();
                let name = second.text().next().unwrap().to_owned();

                curr_header = Some((abbreviation, name, color));
            } else {
                let name_el = element.select(&last_selector).next().unwrap();
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
                    .unwrap()
                    .text()
                    .next()
                    .unwrap();

                let (kind, hours) = details.split_once(' ').unwrap();

                let kind = ClassKind::from(kind.to_lowercase().as_str());
                let hours: u8 = hours.parse().unwrap();

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

    subjects
}

fn parse_classes(table: &ElementRef) -> HashMap<chrono::NaiveDate, [Option<Class>; 7]> {
    let header_selector = Selector::parse("tr:first-child>td").unwrap();

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
    let mut offsets = [0; 55];

    for col in 0..total_columns {
        for weekday in 0..7 {
            let day_header_selector = Selector::parse(&format!(
                "tr:nth-child({})>td:nth-child({})",
                2 + weekday * 8,
                3 + col
            ))
            .unwrap();
            let day_header = table.select(&day_header_selector).next().unwrap();
            if matches!(day_header.attr("background"), Some("outofrange.gif")) {
                continue;
            }

            let (day, month) = day_header.text().next().unwrap().split_once(' ').unwrap();
            let day = day.parse::<u32>().unwrap();
            let month = month_from_roman(month).unwrap();
            let year = if month > 9 { 2026 } else { 2027 };
            let date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();

            let mut classes: [Option<Class>; 7] = Default::default();
            for (i, item) in classes.iter_mut().enumerate() {
                let row = weekday * 8 + i;

                let class_selector = Selector::parse(&format!(
                    "tr:nth-child({})>td:nth-child({})",
                    row + 2,
                    2 + col - offsets[row]
                ))
                .unwrap();
                let class = table.select(&class_selector).next().unwrap();
                if class.attr("bgcolor").is_some() {
                    if let Some(cols) = class.attr("colspan").and_then(|v| v.parse::<u8>().ok()) {
                        offsets[row] += cols - 1;
                    }

                    if let Some(rows) = class.attr("rowspan").and_then(|v| v.parse::<usize>().ok())
                    {
                        for offset in offsets.iter_mut().skip(row).take(rows) {
                            *offset += 1;
                        }
                    }

                    let mut values = class.text();
                    let code = values.next().unwrap().to_owned();
                    let kind = values.next().map(ClassKind::from);
                    let rest = values.collect::<String>();

                    let room = if rest.trim().is_empty() {
                        None
                    } else {
                        Some(rest)
                    };

                    *item = Some(Class { code, kind, room });
                }
            }

            if classes.iter().any(Option::is_some) {
                result.insert(date, classes);
            }
        }
    }

    result
}

fn month_from_roman(value: &str) -> Option<u32> {
    match value {
        "I" => Some(1),
        "II" => Some(2),
        "III" => Some(3),
        "IV" => Some(4),
        "V" => Some(5),
        "VI" => Some(6),
        "VII" => Some(7),
        "VIII" => Some(8),
        "IX" => Some(9),
        "X" => Some(10),
        "XI" => Some(11),
        "XII" => Some(12),
        _ => None,
    }
}
