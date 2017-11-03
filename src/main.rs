extern crate cidr;
use cidr::Ipv4Cidr;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

fn cidrs_list() -> Vec<Ipv4Cidr> {
    vec![
        Ipv4Cidr::from_str("10.1.9.32/16").unwrap()
    ]
}
