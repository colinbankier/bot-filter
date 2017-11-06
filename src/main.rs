extern crate cidr;
extern crate treebitmap;

use treebitmap::{IpLookupTable, IpLookupTableOps};
use cidr::{Ipv4Cidr, Cidr};

use std::str::FromStr;
use std::net::Ipv4Addr;
use std::{env, process};
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, Error, ErrorKind};
use std::num;
use std::io::BufReader;

#[derive(Debug)]
enum ParseError {
    Io(io::Error),
    ParseInt(num::ParseIntError),
}

#[derive(Debug)]
struct Event {
    ip_address: String,
    session_id: i32
}

impl FromStr for Event {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split('\t').collect();
        let session_string = v.get(0).ok_or(Error::new(ErrorKind::Other, format!("Couldn't parse line: {}", s))).map_err(ParseError::Io)?;
        let ip_address = v.get(1).ok_or(Error::new(ErrorKind::Other, format!("Couldn't parse line: {}", s))).map_err(ParseError::Io)?;
        let session_id = session_string.parse().map_err(ParseError::ParseInt)?;
        Ok(Event {
            ip_address: ip_address.to_string(),
            session_id: session_id
        })
    }
}

struct Args {
    events_filename: String,
    cidr_filename: String
}

impl Args {
    fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let events_filename = args[1].clone();
        let cidr_filename = args[2].clone();

        Ok(Args{cidr_filename, events_filename})
    }
}

fn main() {
    let cmd_args: Vec<String> = env::args().collect();
    let args = Args::new(&cmd_args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let cidrs = cidrs_tree(&args.cidr_filename);
    let file = File::open(&args.events_filename).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    });
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        match line {
            Ok(event_str) => {
                process_line(&event_str, &cidrs)
            },
            Err(err) => eprintln!("Error reading line {}", err)
        }
    }
}

fn process_line(event_str: &str, cidrs: &IpLookupTable<Ipv4Addr, String>) {
    match Event::from_str(&event_str) {
        Ok(event) => {
            match event.ip_address.parse() {
                Ok(ip) => {
                    if cidrs.longest_match(ip).is_some() {
                        println!("{}", event.session_id);
                    }
                },
                Err(err) => {
                    eprintln!("Invalid ip address {} {}", event.ip_address, err);
                }
            }
        },
        Err(err) => {
            eprintln!("Invalid event {:?}", err);
        }
    }
}

fn cidrs_tree(filename: &str) -> IpLookupTable<Ipv4Addr, String> {
    let lines = read_file_lines(filename).unwrap_or_else(|err| {
        eprintln!("Problem reading cidrs: {}", err);
        process::exit(1);
    });
    let mut cidrs = IpLookupTable::new();
    for line in lines {
        match Ipv4Cidr::from_str(&line) {
            Ok(cidr) => {
                cidrs.insert(cidr.first_address(), cidr.network_length() as u32, line);
            },
            Err(err) => {
                eprintln!("Invalid cidr {} {}", line, err);
            }
        };
    }
    cidrs
}

fn read_file_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename).expect("file not found");
    let buf_reader = BufReader::new(file);
    buf_reader.lines().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use treebitmap::{IpLookupTable, IpLookupTableOps};

    #[test]
    fn test_overlapping_cidrs() {
        let mut cidrs = IpLookupTable::new();
        for address in vec!["10.10.10.0/24", "10.10.0.0/16", "10.9.199.128/25", "10.11.0.0/16"] {
            let cidr = Ipv4Cidr::from_str(address).unwrap();
            cidrs.insert(cidr.first_address(), cidr.network_length() as u32, address);
        }

        // Existing IPs
        assert!(cidrs.longest_match("10.10.255.253".parse().unwrap()).is_some());
        assert!(cidrs.longest_match("10.11.3.3".parse().unwrap()).is_some());

        // Non-existing IPs
        assert!(cidrs.longest_match("10.12.255.253".parse().unwrap()).is_none());
    }
}
