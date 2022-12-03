pub fn proc_part1(input: &str) -> String {
    let res = input
        .lines()
        .map(|rucksack| rucksack.chars().collect::<Vec<_>>())
        .map(|rucksack| {
            let l: usize = rucksack.len() / 2;
            let mut res = rucksack[0..l]
                .iter()
                .filter(|&c| rucksack[l..(2 * l)].iter().any(|c1| c1 == c))
                .map(|c| c.clone())
                .collect::<Vec<_>>();

            res.dedup();
            res
        })
        .map(|wrong| {
            wrong
                .iter()
                .map(|wchar| {
                    if wchar.is_uppercase() {
                        wchar.clone() as u32 - 38
                    } else {
                        wchar.clone() as u32 - 96
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    res.to_string()
}

pub fn proc_part2(input: &str) -> String {
    let res = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|squad| {
            squad
                .iter()
                .map(|rucksack| rucksack.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(|squad| {
            squad[0]
                .iter()
                .filter(|&c| squad[1].iter().any(|c1| c1 == c) && squad[2].iter().any(|c1| c1 == c))
                .map(|c| c.clone())
                .collect::<Vec<_>>()
        })
        .map(|mut sq| {
            sq.dedup();
            sq
        })
        .map(|wrong| {
            wrong
                .iter()
                .map(|wchar| {
                    if wchar.is_uppercase() {
                        wchar.clone() as u32 - 38
                    } else {
                        wchar.clone() as u32 - 96
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(proc_part1(INPUT), "157");
    }

    #[test]
    fn test_part2() {
        assert_eq!(proc_part2(INPUT), "70");
    }
}
