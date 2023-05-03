use std::fs::File;
use std::io::{prelude::*};

pub fn read_file(filepath: &str) -> Vec<(i32, i32)>{
    let file = File::open(filepath).expect("could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    let mut data:Vec<(i32,i32)> = Vec::new();
    for line in buf_reader{
        let line_str = line.expect("error reading");
        let v = line_str.trim();
        let mut _temp:Vec<&str> = Vec::new();
        if line_str.starts_with("#")==false{
            _temp = v.split('\t').collect();
            let from = _temp[0].parse::<i32>().unwrap();
            let to = _temp[1].parse::<i32>().unwrap();
            data.push((from, to));
        }
    }
    return data;
}