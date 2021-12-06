use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq)]
enum Change {
    Same,
    Increase,
    Decrease
}

fn main() {    
    let path = env::args()
        .nth(1).expect("path to file missing");

    let file = File::open(path).expect("cannot open file");
    let reader = BufReader::new(file);

    let measurements: Vec<u32> = reader.lines()
        .map(|line| line
            .expect("error reading line")
            .parse()
            .expect("error parsing number")
        )
        .collect();

    part_one(&measurements);
    part_two(&measurements);
}

fn part_one(measurements: &Vec<u32>) {
    let bottom = measurements.iter();
    let top = measurements.iter().skip(1);
    let changes = bottom.zip(top).map(|(b, t)| {
        if t > b {
            Change::Increase
        } else {
            Change::Decrease
        }
    });

    let increased = changes.filter(|c| *c == Change::Increase).count();
    println!("part one: {}", increased);
}

fn part_two(measurements: &Vec<u32>) {
    let bottom = 0..=measurements.len() - 3;
    let top = 3..=measurements.len();

    let groups: Vec<u32> = bottom
        .zip(top)
        .map(|(b, t)| {
            *(&measurements[b..t].iter().sum::<u32>())
        })
        .collect();

    let bottom = groups.iter();
    let top = groups.iter().skip(1);
    let changes = bottom.zip(top).map(|(b, t)| {
        if t > b {
            Change::Increase
        } else if t == b {
            Change::Same
        } else {
            Change::Decrease
        }
    });

    let increased = changes.filter(|c| *c == Change::Increase).count();
    println!("part two: {}", increased);
}