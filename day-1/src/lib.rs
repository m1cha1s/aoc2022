pub fn proc_part1(input: &str) -> String {
    let res = input
        .split("\n\n")
        .map(|elf_pkg| {
            elf_pkg
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    res.to_string()
}

pub fn proc_part2(input: &str) -> String {
    let mut res = input
        .split("\n\n")
        .map(|elf_pkg| {
            elf_pkg
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    res.sort_by(|a, b| b.cmp(a));

    res[0..3].iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!(proc_part1(INPUT), "24000");
    }

    #[test]
    fn test_part2() {
        assert_eq!(proc_part2(INPUT), "45000");
    }
}
