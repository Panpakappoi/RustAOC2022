use std::fs::File;
// use std::io::prelude::*;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "Z:\\DL\\data.text";
    // let file = match File::open(file_path){
    //     Ok(file) => file,
    //     Err(error) => panic!("Error opening file: {}", error),
    // };
    let file = File::open(file_path).expect("Error Opening file");
    let reader = BufReader::new(file);

    let mut group_sum = 0;
    let mut group_sums = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.trim().is_empty() {
            group_sums.push(group_sum);
            group_sum = 0;
        } else {
            let value: i32 = line.trim().parse().expect("Error reading line.");
            group_sum += value;
        }
    }
    group_sums.sort_by(|a, b| b.cmp(a));
    for sum in &group_sums {
        println!("{}", sum);
    }
    let highest = group_sums[0];
    println!("Highest value: {}", highest);
    let sum_of_three = group_sums[0] + group_sums[1] + group_sums[2];
    println!("Highest three{}", sum_of_three);
}