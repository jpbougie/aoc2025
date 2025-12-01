use std::ops::Range;

use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let input = parse_input(include_str!("../inputs/02/input.txt"))?;

    let part1 = part_01(&input);
    println!("Part 01: {part1}");
    let part2 = part_02(&input);
    println!("Part 02: {part2}");
    Ok(())
}

fn resolver<F>(input: &Vec<Range<u64>>, validator: F) -> u64
where
    F: Fn(&u64) -> bool,
{
    input
        .iter()
        .map(|range| range.clone().filter(&validator).sum::<u64>())
        .sum()
}

fn part_01(input: &Vec<Range<u64>>) -> u64 {
    resolver(input, invalid_id_part1)
}

fn invalid_id_part1(n: &u64) -> bool {
    let count = count_digits(*n);
    if count % 2 == 1 {
        return false;
    }

    let tens = 10u64.pow((count / 2) as u32);
    n / tens == n % tens
}

fn part_02(input: &Vec<Range<u64>>) -> u64 {
    resolver(input, invalid_id_part2)
}

fn invalid_id_part2(n: &u64) -> bool {
    let digits = count_digits(*n);
    if digits < 2 {
        return false;
    }
    for group in (1..=digits / 2).rev() {
        if digits % group != 0 {
            continue;
        }
        let mut groups = GroupsOf::new(*n, group as u32).into_iter();
        let Some(pivot) = groups.next() else {
            return false;
        };
        if groups.all(|i| i == pivot) {
            return true;
        }
    }

    false
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Range<u64>>> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|pair| {
            let Some((from, to)) = pair.split_once('-') else {
                return Err(anyhow!("Bad pair: {pair}"));
            };
            let from = from.parse()?;
            let to = to.parse()?;
            Ok((from..to))
        })
        .collect::<_>()
}

fn count_digits(n: u64) -> u64 {
    (n.checked_ilog10().unwrap_or(0) as u64) + 1
}

struct GroupsOf {
    n: u64,
    digits: u32,
}

impl GroupsOf {
    fn new(n: u64, digits: u32) -> Self {
        Self { n, digits }
    }
}

impl Iterator for GroupsOf {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return None;
        }

        let tens = 10u32.pow(self.digits) as u64;
        let r = self.n % tens;
        self.n /= tens;

        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_id_part1() {
        assert_eq!(true, invalid_id_part1(&55));
        assert_eq!(false, invalid_id_part1(&5));
        assert_eq!(false, invalid_id_part1(&54));
    }

    #[test]
    fn test_groups_of() {
        assert_eq!(
            vec![123, 123, 123],
            GroupsOf::new(123123123, 3).into_iter().collect::<Vec<_>>()
        )
    }

    #[test]
    fn test_invalid_id_part2() {
        assert_eq!(true, invalid_id_part2(&55));
        assert_eq!(false, invalid_id_part2(&5));
        assert_eq!(false, invalid_id_part2(&54));
        assert_eq!(true, invalid_id_part2(&545454));
        assert_eq!(true, invalid_id_part2(&824824824));
    }
}
