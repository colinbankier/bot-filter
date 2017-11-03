extern crate cidr;
use cidr::{Ipv4Cidr, Cidr};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::{env, process};
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
struct Event {
    source_ip: Ipv4Addr,
    session_id: String
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
    println!("Hello, world!");

    let cmd_args: Vec<String> = env::args().collect();
    let args = Args::new(&cmd_args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let cidrs = cidrs_list(&args.cidr_filename);
    let events = get_events(&args.events_filename);
    let filtered = events.iter().filter(|event|
        cidrs.iter().any(|cidr| cidr.contains(&event.source_ip))
    );
    for event in filtered {
        println!("{}", event.session_id);
    }
}

fn cidrs_list(filename: &str) -> Vec<Ipv4Cidr> {
    vec![
        Ipv4Cidr::from_str("10.1.0.0/16").unwrap(),
        Ipv4Cidr::from_str("10.2.0.0/16").unwrap()
    ];
    let cidrs = read_file(filename)


}

fn read_file_lines(filename: &str) -> String {
    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

fn get_events(filename: &str) -> Vec<Event> {
    vec![
        Event{
            source_ip: "10.1.9.1".parse().unwrap(),
            session_id: "1".to_string()
        }
    ]
}
