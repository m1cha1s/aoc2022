enum Outcome {
    Win(i32),
    Draw(i32),
    Loss(i32),
}

pub fn proc_part1(input: &str) -> String {
    let res = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|action| match action {
                    "A" | "X" => 1,
                    "B" | "Y" => 2,
                    "C" | "Z" => 3,
                    _ => 0,
                })
                .collect::<Vec<i32>>()
        })
        .map(|guide| match (guide[0], guide[1]) {
            (1, 1) => Outcome::Draw(guide[1]),
            (1, 2) => Outcome::Win(guide[1]),
            (1, 3) => Outcome::Loss(guide[1]),
            (2, 1) => Outcome::Loss(guide[1]),
            (2, 2) => Outcome::Draw(guide[1]),
            (2, 3) => Outcome::Win(guide[1]),
            (3, 1) => Outcome::Win(guide[1]),
            (3, 2) => Outcome::Loss(guide[1]),
            (3, 3) => Outcome::Draw(guide[1]),
            (_, _) => Outcome::Loss(0),
        })
        .map(|outcome| match outcome {
            Outcome::Win(a) => 6 + a,
            Outcome::Draw(a) => 3 + a,
            Outcome::Loss(a) => a,
        })
        .sum::<i32>();

    res.to_string()
}

pub fn proc_part2(input: &str) -> String {
    let res = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|action| match action {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    "X" => 0,
                    "Y" => 3,
                    "Z" => 6,
                    _ => 0,
                })
                .collect::<Vec<i32>>()
        })
        .map(|guide| match guide[1] {
            0 => Outcome::Loss(guide[0] + 2),
            3 => Outcome::Draw(guide[0]),
            6 => Outcome::Win(guide[0] + 1),
            _ => Outcome::Loss(0),
        })
        .map(|outcome| match outcome {
            Outcome::Win(a) => Outcome::Win(if a > 3 { a - 3 } else { a }),
            Outcome::Draw(a) => Outcome::Draw(if a > 3 { a - 3 } else { a }),
            Outcome::Loss(a) => Outcome::Loss(if a > 3 { a - 3 } else { a }),
        })
        .map(|outcome| match outcome {
            Outcome::Win(a) => 6 + a,
            Outcome::Draw(a) => 3 + a,
            Outcome::Loss(a) => a,
        })
        .sum::<i32>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(proc_part1(INPUT), "15");
    }

    #[test]
    fn test_part2() {
        assert_eq!(proc_part2(INPUT), "12");
    }
}
