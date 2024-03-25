use std::convert::Into;
use std::io::Write;
use std::string::ToString;

fn solution_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut n = line.chars().filter_map(|it| {
                if it >= '0' && it <= '9' {
                    Some(it as u8 - '0' as u8)
                } else {
                    None
                }
            });
            let a = n.next().unwrap();
            let b = n.last().unwrap_or(a);
            a as u32 * 10 + b as u32
        })
        .sum::<u32>()
}

fn word_to_number(word: &str, mapping: &Vec<(&str, u8)>) -> Option<u8> {
    for (k, v) in mapping {
        if word.contains(k) {
            return Some(*v);
        }
    }
    None
}

pub fn solution_2(input: &str) -> u32 {
    input
        .lines()
        .map(|str| {
            let mut ite = str.chars().enumerate().filter_map(|(i, c)| {
                if c.is_digit(10) {
                    Some(c as u8 - '0' as u8)
                } else {
                    match &str[i..] {
                        s if s.starts_with("one") => Some(1u8),
                        s if s.starts_with("two") => Some(2),
                        s if s.starts_with("three") => Some(3),
                        s if s.starts_with("four") => Some(4),
                        s if s.starts_with("five") => Some(5),
                        s if s.starts_with("six") => Some(6),
                        s if s.starts_with("seven") => Some(7),
                        s if s.starts_with("eight") => Some(8),
                        s if s.starts_with("nine") => Some(9),
                        _ => None,
                    }
                }
            });
            let a = ite.next().unwrap();
            let b = ite.last().unwrap_or(a);
            a as u32 * 10 + b as u32
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("input.txt");
        dbg!(solution_1(input));
    }

    #[test]
    fn test2() {
        let input = include_str!("input.txt");
        dbg!(solution_2(input));
    }
}
