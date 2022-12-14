mod db;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::db::DTYPE;

fn process_table<P>(filename: P, rows: i32) -> String 
        where P: AsRef<Path>, {

    let mut out_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("out.sql")
        .unwrap();
    
    
    // random generator
    let mut rng = rand::thread_rng();
    // read from given file
    if let Ok(lines) = read_lines(filename) {
        let mut first_row = true;
        let mut table = db::Table{name: String::from(""), fields: vec![]};
        // parse the table file
        for line in lines {
            if let Ok(line_) = line {
                if first_row {
                    // try to extract table name
                    let re = Regex::new(r"\s*CREATE\s*TABLE\s*(\w+)\s*\(").unwrap();
                    for cap in re.captures_iter(line_.as_str()) {
                        println!("table_name: {} ", &cap[1]);
                        table.name = cap[1].to_string();
                    }
                    first_row = false;
                } else {
                    // try to extract field
                    let re = Regex::new(r"\s*(\w+)\s*(\w+)\s*,").unwrap();
                    for cap in re.captures_iter(line_.as_str()) {
                        println!("name: {} type: {}", &cap[1], &cap[2]);
                        let dtype = DTYPE::from_s((&cap[2]).to_string());
                        if dtype.is_none() {
                            return "invalid data type".to_string()
                        }
                        table.fields.push(db::Field{name: (&cap[1]).to_string(), 
                            dtype: dtype.unwrap()})
                    }
                }
            } 
        }
        // generate SQL INSERT with format as:
        // INSERT INTO table_name (column1, column2, column3, ...)
        // VALUES (value1, value2, value3, ...);
        let mut header: String = format!("INSERT INTO {} ", table.name).to_owned();
        // construct column names
        header.push_str("(");
        for f_num in 0..table.fields.len() {
            if f_num != 0 {
                header.push_str(",");
            } 
            header.push_str(&table.fields[f_num].name);
        }
        for _ in 0..rows {
            let mut body: String = ") VALUES (".to_owned();
            for f_num in 0..table.fields.len() {
                if f_num != 0 {
                    body.push_str(",");
                }
                if table.fields[f_num].dtype == DTYPE::FLOAT {
                    body.push_str(format!("{}", rng.gen::<i32>()).as_str());
                } else {
                    body.push_str(format!("{}", rng.gen::<f64>()).as_str());
                }
            }
            body.push_str(");");
            println!("sql: {} {}", header, body);
            if let Err(e) = writeln!(out_file, "{} {}", header, body) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        "".to_string()
    } else {
        "failed to read from file".to_string()
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let res = process_table("create_table.sql", 10);
    if res == "" {
        println!("sucess!")
    } else {
        println!("failed: {}", res);
    }
    
}
