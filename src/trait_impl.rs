use std::{str::FromStr,result, error::Error, io::ErrorKind};

use chrono::{NaiveDate, ParseError};
use serde::{de,Serialize,Deserialize, Serializer, Deserializer};

#[derive(Debug, Clone, Copy)]
pub struct Date(NaiveDate);
impl Date {
    pub fn value(&self) -> NaiveDate {
        self.0
    }
}

impl ToString for Date {
    fn to_string(&self) -> String {
        self.value().format("%Y-%m-%d").to_string()
    }
}

impl FromStr for Date {
    type Err = ParseError;
    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|err| err.to_string()).unwrap();
        Ok(Self(date))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Date", &self.to_string())
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}
