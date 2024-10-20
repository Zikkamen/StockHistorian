mod file_reader;
mod values_store;

use std::fs::File;
use std::net::TcpStream;
use std::io::{Write, Read};

use crate::values_store::credentials_store::CredentialsStore;
use crate::values_store::stock_data::StockData;

#[tokio::main]
async fn main() {
    let credentials_store = CredentialsStore::new();

    let apikey = credentials_store.get_token("twelvedata.com");

    get_data("RHM", apikey).await;
}

async fn get_data(stock_name: &str, apikey: String) {
    let client = reqwest::Client::new();

    let response = match client.get(format!("https://api.twelvedata.com/time_series?symbol={}&interval=5min&&apikey={}&outputsize=5000&format=csv&exchange=FSX", stock_name, apikey)).send().await {
        Ok(v) => v,
        Err(e) => panic!("Error getting data: {}", e),
    };

    let body = match response.text().await {
        Ok(v) => v,
        Err(e) => panic!("Error parsing body: {}", e),
    };

    let data = parse_csv_file(body);

    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    stream.set_read_timeout(None).expect("set_read_timeout call failed");
    stream.write(data.as_bytes()).unwrap();

    let mut result = String::new();
    stream.read_to_string(&mut result).unwrap();

    println!("{result}");
}

fn parse_csv_file(body: String) -> String {
    let data = body.chars();

    let mut lines:String = String::new();
    let mut row:Vec<String> = Vec::new();
    let mut tmp:String = String::new();

    let mut header = true;

    for c in data.into_iter() {
        match c {
            '\n' => {
                if tmp.len() != 0 { 
                    row.push(tmp);
                    tmp = String::new();
                }

                if row.len() == 6 {
                    if !header {
                        lines.push_str(&format!("{};{}\n", row[4], row[5]));
                    }

                    header = false;
            
                    row = Vec::new();
                }
            },
            ';' => {
                row.push(tmp);
                tmp = String::new();
            },
            _ => tmp.push(c),
        };
    }

    data_payload(lines)
}

fn data_payload(data: String) -> String {
    let status_line = "GET / HTTP/1.1";
    let length = data.len();

    return format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{data}\0");
}