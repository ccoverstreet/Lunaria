use chrono::{NaiveDate, Datelike};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BudgetEntry {
    pub date: NaiveDate,
    pub name: String,
    pub val: f64,
    pub tags: Vec<String>
}

impl BudgetEntry {
    pub fn new(date_str: String, name: String, val: f64, tags: Vec<String>) -> Result<BudgetEntry, chrono::ParseError> {
        Ok(BudgetEntry{
            date: NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")?,
            name: name,
            val: val,
            tags: tags
        })
    }

    pub fn month(&self) -> u32 {
        self.date.month()
    }

    pub fn year(&self) -> i32 {
        return self.date.year()
    }
}

