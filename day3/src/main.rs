extern crate regex;

use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;

fn main() {
    // Claim board
    const SIZE: usize = 1000 * 1000;
    let mut claims: Vec<Vec<u32>> = Vec::with_capacity(SIZE);

    // init vector
    for _ in 0..SIZE {
        let vec: Vec<u32> = Vec::new();
        claims.push(vec);
    }

    let file = File::open("input.txt").expect("cannot read inputs");
    let file = BufReader::new(file);

    let reg = Regex::new(r"#(\d*) @ (\d*),(\d*): (\d*)x(\d*)").unwrap();

    let lines: Vec<_> = file.lines().collect();
    let linecount = lines.len();

    let mut uncontested: Vec<bool> = Vec::new();
    for _ in 0..linecount+1 {
        uncontested.push(true);
    }

    // Load claims, add 1 for each claim
    for line in lines {
        for cap in reg.captures_iter(&line.unwrap()) {
            let id:     u32 = cap[1].parse().unwrap();
            let left:   i32 = cap[2].parse().unwrap();
            let top:    i32 = cap[3].parse().unwrap();
            let width:  i32 = cap[4].parse().unwrap();
            let height: i32 = cap[5].parse().unwrap();

            for x in 0..width {
                for y in 0..height {
                    let loc = get_location(left + x, top + y);
                    claims[loc].push(id);

                    if claims[loc].len() > 1 {
                        for i in 0..claims[loc].len() {
                            let ex_id = claims[loc][i];
                            uncontested[ex_id as usize] = false;
                        }
                    }
                }
            }
        }
    }

    // Claims are loaded, count squares with more than 1 claim
    let mut contested: u32 = 0;

    for claim in claims.iter() {
        if claim.len() > 1 {
            contested += 1;
        }
    }

    println!("Contested count: {}", contested);

    for i in 0..uncontested.len() {
        if i != 0 && uncontested[i] != false {
            println!("Uncontested ID: {}", i);
        }
    }
}


fn get_location(x: i32, y: i32) -> usize {
    let s: usize = (x + (y * 1000)) as usize;
    return s;
}
