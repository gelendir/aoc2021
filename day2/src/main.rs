use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq, Debug)]
enum Operation {
    Forward(i32),
    Down(i32),
    Up(i32)
}

fn main() {    
    let path = env::args()
        .nth(1).expect("path to file missing");

    let file = File::open(path).expect("cannot open file");
    let reader = BufReader::new(file);

    let operations: Vec<Operation> = reader.lines()
    .map(|result| {
        let line = result.expect("error readling line");
        let mut parts = line.split(" ");

        let operation = parts.next().expect("error reading operation");
        let value: i32 = parts.next()
            .expect("error reading value")
            .parse()
            .expect("error parsing integer");

        match operation {
            "forward" => Operation::Forward(value),
            "down" => Operation::Down(value),
            "up" => Operation::Up(value),
            e => panic!("unexpected operation {}", e)
        }
    })
    .collect();

    part_one(&operations);
    part_two(&operations);
}

fn part_one(operations: &Vec<Operation>) {
    let position: i32 = operations.iter()
        .filter_map(|op| match op {
            Operation::Forward(v) => Some(v),
            _ => None
        })
        .sum();

    let depth: i32 = operations.iter()
        .filter_map(|op| match op {
            Operation::Down(v) => Some(*v),
            Operation::Up(v) => Some(-*v),
            _ => None
        })
        .sum();

    println!("part one - position: {}, depth: {}, result: {}", position, depth, position*depth);
}

fn part_two(operations: &Vec<Operation>) {
    let (final_position, final_depth, _) = operations.iter()
        .fold((0, 0, 0), |(pos, depth, aim), operation| {
            match operation {
                Operation::Forward(value) => {
                    let pos = pos + value;
                    let depth = depth + (value * aim);
                    (pos, depth, aim)
                },
                Operation::Down(value) => (pos, depth, aim + value),
                Operation::Up(value) => (pos, depth, aim - value)
            }
        });

    println!("part two - position: {}, depth: {}, result: {}", final_position, final_depth, final_position * final_depth);
}