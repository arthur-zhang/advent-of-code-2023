fn solution() {
    let str = include_str!("input.txt");
    let result = str
        .lines()
        .map(|line| {
            let mut n = line
                .chars()
                .filter(|it| *it >= '0' && *it <= '9')
                .map(|it| it as u8 - '0' as u8);
            let a = n.next().unwrap();
            let b = n.rev().next().unwrap_or(a);
            a as u32 * 10 + b as u32
        })
        .sum::<u32>();
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        solution();
    }
}
