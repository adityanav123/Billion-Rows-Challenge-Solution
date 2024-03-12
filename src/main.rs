extern crate clap;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

use clap::{Error, Parser};

#[derive(Debug, PartialOrd, PartialEq)]
struct CityStats {
    min_temp: f64,
    max_temp: f64,
    sum: f64,
    count: usize, // unsigned int , width = cpu pointer
}

impl Default for CityStats {
    fn default() -> Self {
        Self {
            min_temp: f64::INFINITY,
            max_temp: f64::NEG_INFINITY,
            sum: Default::default(),
            count: Default::default(),
        }
    }
}

// Command Line Argument
#[derive(Debug, Parser)]
#[command(version="1.0.0", about="Solution to Billion Rows Challenge in Rust", long_about=None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let parsed_file = Args::parse();
    
    // taking file name as cli argument
    if parsed_file.filename.is_empty() {
        exit(0);
    }

    let path = parsed_file.filename;
    let f = File::open(path).unwrap();
    let f = BufReader::new(f); // rust maintains a buffered io

    let mut store = BTreeMap::<String, CityStats>::new(); // auto sort

    for line in f.lines().map_while(Result::ok).take(1_000_000_000) {
        // string format : Hamburg;21
        if let Some((city, temp)) = line.split_once(';') {
            let temp = temp.parse::<f64>().unwrap();

            let city_data = store.entry(city.to_string()).or_default();

            city_data.max_temp = temp.max(city_data.max_temp);
            city_data.min_temp = temp.min(city_data.min_temp);
            city_data.sum += temp;
            city_data.count += 1;
        } else {
            continue;
        };
    }

    // sorting
    //let mut store = store.into_iter().collect::<Vec<_>>();
    //store.sort_unstable_by(|(city_a, _), (city_b, _)| city_a.cmp(city_b));

    for (city, city_stat) in store {
        let average = city_stat.sum / city_stat.count as f64;
        println!(
            "{} : {}/{}/{}",
            city, city_stat.min_temp, city_stat.max_temp, average
        );
    }
}
