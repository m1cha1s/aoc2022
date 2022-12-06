use std::fs;

use day_6::proc_part1;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", proc_part1(input.as_str()));
}
