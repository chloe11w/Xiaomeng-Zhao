use crate::models::DogBiteRecord;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use chrono::{NaiveDate, Datelike};

pub fn load_data(file_path: &str) -> Result<Vec<DogBiteRecord>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: DogBiteRecord = result?;
        if let Some(zc) = &record.zip_code {
            if !zc.trim().is_empty() {
                records.push(record);
            }
        }
    }

    Ok(records)
}

pub fn extract_month_and_season(date_str: &Option<String>) -> Option<(u32, u32)> {
    if let Some(date_str) = date_str {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, "%B %d %Y") {
            let month = date.month();
            let season = match month {
                12 | 1 | 2 => 1, // Winter
                3 | 4 | 5 => 2, // Spring
                6 | 7 | 8 => 3, // Summer
                9 | 10 | 11 => 4, // Fall
                _ => 0,
            };
            return Some((month, season));
        }
    }
    None
}

pub fn parse_age(age_str: &Option<String>) -> Option<f32> {
    if let Some(age_str) = age_str {
        if let Some(age_num) = age_str.trim_end_matches('Y').parse::<f32>().ok() {
            return Some(age_num);
        }
    }
    None
}
