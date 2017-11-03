extern crate cidr;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use cidr::{Ipv4Cidr, Cidr};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::{env, process};
use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufReader;
use serde_json::Error;

#[derive(Debug, Deserialize)]
struct Event {
    source_ip: String,
    session_id: String
}

impl FromStr for Event {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

struct Args {
    cidr_filename: String,
    events_filename: String
}

impl Args {
    fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let cidr_filename = args[1].clone();
        let events_filename = args[2].clone();

        Ok(Args{cidr_filename, events_filename})
    }
}

fn main() {
    let cmd_args: Vec<String> = env::args().collect();
    let args = Args::new(&cmd_args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let cidrs = cidrs_list(&args.cidr_filename);
    let file = File::open(&args.events_filename).expect("file not found");
    let mut buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        match line {
            Ok(event_str) => {
                match Event::from_str(&event_str) {
                    Ok(event) => {
                        match event.source_ip.parse() {
                            Ok(ip) => {
                                if cidrs.iter().any(|cidr| cidr.contains(&ip)) {
                                    println!("{}", event.session_id);
                                }
                            },
                            Err(err) => {
                                eprintln!("Invalid ip address {}", event.source_ip);
                            }
                        }
                    },
                    Err(err) => {
                        eprintln!("Invalid event {}", err);
                    }
                }
            },
            Err(err) => eprintln!("Error reading line {}", err)
        }
    }
    // let filtered = events.iter().filter(|event|
    //     cidrs.iter().any(|cidr| cidr.contains(&event.source_ip))
    // );
    // for event in filtered {
    //     println!("{}", event.session_id);
    // }
}

fn cidrs_list(filename: &str) -> Vec<Ipv4Cidr> {
    let lines = read_file_lines(filename).unwrap_or_else(|err| {
        eprintln!("Problem reading cidrs: {}", err);
        process::exit(1);
    });
    let mut cidrs = Vec::new();
    for line in lines {
        match Ipv4Cidr::from_str(&line) {
            Ok(cidr) => cidrs.push(cidr.clone()),
            Err(err) => {
                eprintln!("Invalid cidr {} {}", line, err);
            }
        }
    }
    cidrs
}

fn read_file_lines(filename: &str) -> io::Result<Vec<String>> {
    let mut file = File::open(filename).expect("file not found");
    let mut buf_reader = BufReader::new(file);
    buf_reader.lines().collect()
}

// fn line_reader(filename: &str) -> BufReader<String> {
//     let mut file = File::open(filename).expect("file not found");
//     let mut buf_reader = BufReader::new(file);
//     buf_reader
// }

fn get_events(filename: &str) -> Vec<Event> {
    vec![
        Event{
            source_ip: "10.1.9.3".parse().unwrap(),
            session_id: "1".to_string()
        },
        Event{
            source_ip: "11.1.9.1".parse().unwrap(),
            session_id: "2".to_string()
        }
    ]
}
