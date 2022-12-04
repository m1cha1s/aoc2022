pub fn proc_part1(input: &str) -> String {
    let res = input
        .lines()
        .map(|pair| pair.split(",").collect::<Vec<_>>())
        .map(|pair| {
            pair.iter()
                .map(|assignment| {
                    assignment
                        .split("-")
                        .map(|id| id.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|assignment| assignment[0]..=assignment[1])
                .collect::<Vec<_>>()
        })
        .filter(|pair| {
            pair[0].contains(&pair[1].clone().nth(0).unwrap())
                && pair[0].contains(&pair[1].clone().nth_back(0).unwrap())
                || pair[1].contains(&pair[0].clone().nth(0).unwrap())
                    && pair[1].contains(&pair[0].clone().nth_back(0).unwrap())
        })
        .collect::<Vec<_>>()
        .len();

    res.to_string()
}

pub fn proc_part2(input: &str) -> String {
    let res = input
        .lines()
        .map(|pair| pair.split(",").collect::<Vec<_>>())
        .map(|pair| {
            pair.iter()
                .map(|assignment| {
                    assignment
                        .split("-")
                        .map(|id| id.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|assignment| assignment[0]..=assignment[1])
                .collect::<Vec<_>>()
        })
        .filter(|pair| {
            pair[0].contains(&pair[1].clone().nth(0).unwrap())
                || pair[0].contains(&pair[1].clone().nth_back(0).unwrap())
                || pair[1].contains(&pair[0].clone().nth(0).unwrap())
                || pair[1].contains(&pair[0].clone().nth_back(0).unwrap())
        })
        .collect::<Vec<_>>()
        .len();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(proc_part1(INPUT), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(proc_part2(INPUT), "4");
    }
}
