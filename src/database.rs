use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::path::{Path};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File};
use std::io::{Write};
use serde_json::Error;
use rand::{Rng, thread_rng};
use rand::distributions::{Alphanumeric, DistString};
use chrono::{Datelike};

use crate::budget::BudgetEntry;

#[derive(Deserialize, Clone, Debug)]
pub struct AdvancedQuery {
    pub tags: Option<Vec<String>>,
    pub title: Option<String>,
    pub year: Option<i32>,
    pub month: Option<u32>,
    pub day: Option<u32> 
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseEntry {
    id: String,
    data: BudgetEntry,
}

impl DatabaseEntry {
    fn matches_params(&self, params: &AdvancedQuery) -> bool {
        let tag_flag = match &params.tags {
            Some(tags) => {
                if tags.len() == 0 {
                    true
                } else {
                    let mut found = false;
                    'outer: for target in tags.iter() {
                        for t in self.data.tags.iter() {
                            if target == t {
                                found = true;
                                break 'outer;
                            }
                        }
                    }
                    found
                }
            },
            None => true,
        };

        let title_flag = match &params.title {
            Some(title) => {
                let search = title.to_lowercase();
                let target = self.data.name.to_lowercase();
                if search == "" {
                    true
                } else {
                    target.contains(&*search)
                }
            },
            None => true,
        };

        let year_flag = match params.year {
            Some(year) => self.data.date.year() == year,
            None => true,
        };

        let month_flag = match params.month {
            Some(month) => self.data.date.month() == month,
            None => true,
        };

        let day_flag = match params.day {
            Some(day) => self.data.date.day() == day,
            None => true,
        };

        tag_flag && title_flag && year_flag && month_flag && day_flag
    }

}

pub trait BudgetDatabase { 
    fn add_entry(&self, new: BudgetEntry) -> Result<(), ()>;
    fn delete_entry(&self, id: &str) -> Result<(), ()>;
    fn get_by_id(&self, id: &str) -> Option<DatabaseEntry>;
    fn get_by_month(&self, year: i32, month: u32) -> Vec<DatabaseEntry>;
    fn get_by_tags(&self, tags: &Vec<String>) -> Vec<DatabaseEntry>;
    fn get_by_advanced(&self, params: AdvancedQuery) -> Vec<DatabaseEntry>;
}


#[derive(Debug, Clone)]
pub struct JsonDatabase {
    name: String,
    table: Arc<Mutex<HashMap<String, DatabaseEntry>>>
}

impl JsonDatabase {
    pub fn new(name: &str) -> JsonDatabase {
        JsonDatabase{
            name: name.to_string(),
            table: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn from_file(filename: &str) -> Result<JsonDatabase, Error> {
        let name: &str = match Path::new(filename).file_stem() {
            Some(x) => x.to_str().unwrap(),
            None => filename
        };
        let data = &fs::read_to_string(filename).unwrap_or("{}".to_string());
        let inner: Mutex<HashMap<String, DatabaseEntry>> = 
            serde_json::from_str(data)?;

        Ok(JsonDatabase{
            name: name.to_string(),
            table: Arc::new(inner),
        })
    }

    pub fn from_str(name: &str, json_str: &str) -> Result<JsonDatabase, Error> {
        let inner: Mutex<HashMap<String, DatabaseEntry>> = serde_json::from_str(json_str)?;
        Ok(JsonDatabase{
            name: name.to_string(),
            table: Arc::new(inner)
        })
    }

    pub fn save(name: &str, table: HashMap<String, DatabaseEntry>) -> std::io::Result<()> {
        let out = serde_json::to_string(&table.clone())?;

        let mut file = File::create(&format!("{}.json", name))?;

        file.write_all(out.as_bytes())?;

        Ok(())
    }
}

impl BudgetDatabase for JsonDatabase {
    fn add_entry(&self, new: BudgetEntry) -> Result<(),()> {
        let mut table = self.table.lock().unwrap();
        let mut rng = thread_rng();
                 
        // Find available key
        let mut key: String = "".to_string();
        loop {
            let test_key = Alphanumeric.sample_string(&mut rng, 32);
            let contains = table.contains_key(&test_key.clone());
            if !contains { 
                key = test_key;
                break;
            }
        }

        table.insert(key.clone(), DatabaseEntry{
            id: key,
            data: new
        });

        Self::save(&self.name, table.clone());

        Ok(())
    }

    fn delete_entry(&self, id: &str) -> Result<(), ()> {
        let mut table = self.table.lock().unwrap(); 

        match table.remove(id)  {
            Some(_) => {
                Self::save(&self.name, table.clone());
                Ok(())
            },
            None => Err(())
        }
    }

    fn get_by_id(&self, id: &str) -> Option<DatabaseEntry> {
        let table = self.table.lock().unwrap();
        let data = table.get(id)?;
        Some(data.clone())
    }

    fn get_by_month(&self, year: i32, month: u32) -> Vec<DatabaseEntry> {
        let mut out: Vec<DatabaseEntry> = vec!();
        let table = self.table.lock().unwrap();

        for (_, val) in table.iter() {
            if val.data.year() == year
                && val.data.month() == month {
                out.push(val.clone());
            }
        }

        out
    }

    fn get_by_tags(&self, tags: &Vec<String>) -> Vec<DatabaseEntry> {
        let table = self.table.lock().unwrap();
        let mut out: Vec<DatabaseEntry> = vec!();

        for (_, val) in table.iter() {
            'outer: for vtag in val.data.tags.iter() {
                for stag in tags.iter() {
                    if vtag == stag {
                        out.push(val.clone());
                        break 'outer;
                    }
                }
            }
        }
        
        out
    }

    fn get_by_advanced(&self, params: AdvancedQuery) -> Vec<DatabaseEntry> {
        let table = self.table.lock().unwrap();
        let mut out: Vec<DatabaseEntry> = vec!();

        for (_, val) in table.iter() {
            match val.matches_params(&params) {
                true => out.push(val.clone()),
                false => continue
            }
        }

        out
    }

}
