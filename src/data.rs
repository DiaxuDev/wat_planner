use std::{collections::HashMap, fmt::Debug, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize, de::Visitor};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub legend: HashMap<String, Subject>,
    pub classes: HashMap<chrono::NaiveDate, [Option<Class>; 7]>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub code: String,
    pub kind: Option<ClassKind>,
    pub room: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    pub name: String,
    pub color: Color,
    pub professors: HashMap<ClassKind, Vec<Professor>>,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum ClassKind {
    Lecture,
    Practice,
    Lab,
    Seminar,
    Project,
    Exam,
    MakeUpExam,
    Pass,
    MakeUpPass,
    Retake,
    Unknown(String),
}

impl Serialize for ClassKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Lecture => "lecture",
            Self::Practice => "practice",
            Self::Lab => "lab",
            Self::Seminar => "seminar",
            Self::Project => "project",
            Self::Exam => "exam",
            Self::MakeUpExam => "make_up_exam",
            Self::Pass => "pass",
            Self::MakeUpPass => "make_up_pass",
            Self::Retake => "retake",
            Self::Unknown(inner) => inner,
        })
    }
}

struct ClassKindVisitor;
impl Visitor<'_> for ClassKindVisitor {
    type Value = ClassKind;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("ClassKind enum")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(match v {
            "lecture" => ClassKind::Lecture,
            "practice" => ClassKind::Practice,
            "lab" => ClassKind::Lab,
            "seminar" => ClassKind::Seminar,
            "project" => ClassKind::Project,
            "exam" => ClassKind::Exam,
            "make_up_exam" => ClassKind::MakeUpExam,
            "pass" => ClassKind::Pass,
            "make_up_pass" => ClassKind::MakeUpPass,
            "retake" => ClassKind::Retake,
            other => ClassKind::Unknown(other.to_owned()),
        })
    }
}

impl<'de> Deserialize<'de> for ClassKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ClassKindVisitor)
    }
}

impl From<&str> for ClassKind {
    fn from(value: &str) -> Self {
        match value {
            "w" => Self::Lecture,
            "ć" => Self::Practice,
            "l" => Self::Lab,
            "s" => Self::Seminar,
            "p" => Self::Project,
            "e" => Self::Exam,
            "ep" => Self::MakeUpExam,
            "z" => Self::Pass,
            "zp" => Self::MakeUpPass,
            "x" => Self::Retake,
            _ => Self::Unknown(value.to_owned()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(transparent)]
pub struct Color(u32);

#[allow(clippy::cast_possible_truncation)]
impl Color {
    pub const fn r(self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub const fn g(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub const fn b(self) -> u8 {
        self.0 as u8
    }

    pub fn hex(self) -> String {
        format!("{:06X}", self.0)
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Color")
            .field(&format_args!("#{:06X}", self.0))
            .finish()
    }
}

impl Deref for Color {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Color {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str_radix(s.strip_prefix('#').unwrap_or(s), 16).map(Self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Professor {
    pub name: Option<String>,
    pub hours: u8,
}
