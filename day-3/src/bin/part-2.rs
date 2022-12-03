use std::fs;

use day_3::proc_part2;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", proc_part2(input.as_str()));
}
