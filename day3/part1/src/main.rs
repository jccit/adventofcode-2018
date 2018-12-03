extern crate regex;

use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;

fn main() {
    // Claim board
    const SIZE: usize = 1000 * 1000;
    let mut claims: [u32; SIZE] = [0; SIZE];

    let file = File::open("input.txt").expect("cannot read inputs");
    let file = BufReader::new(file);

    let reg = Regex::new(r"#\d* @ (\d*),(\d*): (\d*)x(\d*)").unwrap();

    // Load claims, add 1 for each claim
    for line in file.lines().filter_map(|result| result.ok()) {
        for cap in reg.captures_iter(&line) {
            let left:   i32 = cap[1].parse().unwrap();
            let top:    i32 = cap[2].parse().unwrap();
            let width:  i32 = cap[3].parse().unwrap();
            let height: i32 = cap[4].parse().unwrap();

            for x in 0..width {
                for y in 0..height {
                    new_claim(&mut claims, left + x, top + y);
                }
            }
        }
    }

    // Claims are loaded, count squares with more than 1 claim
    let mut contested: u32 = 0;

    for claim in claims.iter() {
        if claim > &1 {
            contested += 1;
        }
    }

    println!("{}", contested);
}

fn new_claim(claims: &mut [u32], x: i32, y: i32) {
    let loc = get_location(x, y);
    claims[loc] += 1;
}

fn get_location(x: i32, y: i32) -> usize {
    let s: usize = (x + (y * 1000)) as usize;
    return s;
}
