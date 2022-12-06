use std::str;

pub fn proc_part1(input: &str) -> String {
    for i in 4..input.len() {
        let chars = input[(i - 4)..i].chars();

        let mut same = false;
        for c in chars.clone() {
            if chars
                .clone()
                .filter(|&ch| ch == c)
                .collect::<Vec<_>>()
                .len()
                > 1
            {
                same = true;
            }
        }

        if !same {
            return i.to_string();
        }
    }

    "".to_string()
}

pub fn proc_part2(input: &str) -> String {
    for i in 14..input.len() {
        let chars = input[(i - 14)..i].chars();

        let mut same = false;
        for c in chars.clone() {
            if chars
                .clone()
                .filter(|&ch| ch == c)
                .collect::<Vec<_>>()
                .len()
                > 1
            {
                same = true;
            }
        }

        if !same {
            return i.to_string();
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part1() {
        assert_eq!(proc_part1(INPUT), "7");
    }

    #[test]
    fn test_part2() {
        assert_eq!(proc_part2(INPUT), "19");
    }
}
