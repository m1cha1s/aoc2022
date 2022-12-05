use std::str;

use regex::Regex;

fn parse_inst(raw_inst: String) -> Vec<Vec<usize>> {
    let re = Regex::new(r"move (\d) from (\d) to (\d)").unwrap();
    let res = re
        .captures_iter(raw_inst.as_str())
        .map(|n| {
            vec![
                n[1].parse::<usize>().unwrap(),
                n[2].parse::<usize>().unwrap(),
                n[3].parse::<usize>().unwrap(),
            ]
        })
        .collect::<Vec<_>>();

    res
}

pub fn proc_part1(input: &str) -> String {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let start = sections[0];
    let steps = sections[1];

    let mut rows = start.lines().collect::<Vec<_>>();

    rows.reverse();

    let rows = rows.iter().map(|r| r.to_string()).collect::<Vec<_>>();

    let mut stacks: Vec<Vec<&str>> = Vec::new();

    for i in 0..((rows[1].len() + 1) / 4) {
        stacks.push(Vec::new());
        for row in rows.iter() {
            let container = row.get((1 + (i * 4))..(2 + (i * 4))).unwrap();

            if container != " " && !container.chars().any(|f| f.is_numeric()) {
                stacks[i].push(container);
            }
        }
    }

    println!("{:?}", stacks);

    let instructions = parse_inst(steps.to_string());

    // println!("{:?}", instructions);

    for inst in instructions {
        for _ in 0..inst[0] {
            println!(
                "move {} from {} to {} | {:?}",
                inst[0],
                inst[1] - 1,
                inst[2] - 1,
                stacks[inst[1] - 1]
            );
            let val = stacks[inst[1] - 1].pop().unwrap();
            stacks[inst[2] - 1].push(val);
        }
    }

    // dbg!(stacks);

    let mut res = String::new();

    for stack in stacks {
        res += stack.last().unwrap();
    }

    res
}

pub fn proc_part2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(proc_part1(INPUT), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(proc_part2(INPUT), "");
    }
}
