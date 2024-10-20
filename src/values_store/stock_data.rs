use std::error;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct StockData {
    pub time: i64,
    pub volume: i64,
    pub price_avg: f64,
    pub price_op: f64,
    pub price_cl: f64,
    pub price_hi: f64,
    pub price_lo: f64
}

impl StockData {
    pub fn new(data_row: &Vec<String>) -> Result<Self, Box<dyn error::Error + 'static>> {
        Ok(StockData{
            time: data_row[8].parse::<i64>()?,
            volume: data_row[1].parse::<f64>()? as i64,
            price_avg: data_row[3].parse::<f64>()?,
            price_op: data_row[3].parse::<f64>()?,
            price_cl: data_row[5].parse::<f64>()?,
            price_hi: data_row[6].parse::<f64>()?,
            price_lo: data_row[7].parse::<f64>()?,
        })
    }
}
