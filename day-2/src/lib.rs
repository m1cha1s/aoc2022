enum Outcome {
    Win(i32),
    Draw(i32),
    Loss(i32),
}

pub fn proc_part1(input: String) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "A Y
B X
C Z"
        .to_string();

        assert_eq!(proc_part1(input), "15");
    }
}
