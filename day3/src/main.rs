use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct BitSum {
    zeros: u32,
    ones: u32
}

impl BitSum {

    fn most_common(&self) -> char {
        match self.ones >= self.zeros {
            true => '1',
            false => '0'
        }
    }

    fn least_common(&self) -> char {
        match self.ones < self.zeros {
            true => '1',
            false => '0'
        }
    }

}

fn main() {    
    let path = env::args()
        .nth(1).expect("path to file missing");

    let file = File::open(path).expect("cannot open file");
    let reader = BufReader::new(file);

    let diagnostic: Vec<Vec<char>> = reader.lines()
        .map(|result| {
            result.expect("error readine line")
            .chars()
            .collect()
        })
        .collect();

    part_one(&diagnostic);
    part_two(&diagnostic);
}

fn count_bits(numbers: &Vec<Vec<char>>, column: usize) -> BitSum {
    let (zeros, ones) = numbers.iter()
        .map(|line| line[column])
        .fold((0, 0), |(zeros, ones), bit| match bit {
            '0' => (zeros + 1, ones),
            '1' => (zeros, ones + 1),
            e => panic!("unrecognized bit {}", e)
        });

    BitSum{ zeros, ones }
}

fn part_one(diagnostic: &Vec<Vec<char>>) {
    // we're assuming all lines have the same length here
    let length = diagnostic[0].len();
    let bits: Vec<BitSum> = (0..length)
        .map(|column| count_bits(diagnostic, column))
        .collect();

    let gamma: String = bits.iter()
        .map(|bit| bit.most_common())
        .collect();
    let gamma = u32::from_str_radix(&gamma, 2).expect("cannot parse gamma");

    let epsilon: String = bits.iter()
        .map(|bit| bit.least_common())
        .collect();
    let epsilon = u32::from_str_radix(&epsilon, 2).expect("cannot parse epsilon");

    println!("part one - gamma: {} epsilon: {} result: {}", gamma, epsilon, gamma*epsilon)
}

fn part_two(diagnostic: &Vec<Vec<char>>) {
    let mut column = 0;
    let mut numbers = diagnostic.clone();

    while numbers.len() > 1 {
        let bits = count_bits(&numbers, column);
        numbers.retain(|number| {
            bits.most_common() == number[column]
        });

        column += 1;
    }

    let oxygen: String = numbers[0].iter().collect();
    let oxygen = u32::from_str_radix(&oxygen, 2).expect("cannot parse oxygen");

    column = 0;
    numbers = diagnostic.clone();
    while numbers.len() > 1 {
        let bits = count_bits(&numbers, column);
        numbers.retain(|number| {
            bits.least_common() == number[column]
        });
        column += 1;
    }

    let co2: String = numbers[0].iter().collect();
    let co2 = u32::from_str_radix(&co2, 2).expect("cannot parse co2");

    println!("part two - oxygen: {} co2: {} result: {}", oxygen, co2, oxygen * co2)    
}