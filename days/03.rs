use anyhow::{Result, anyhow};

type Input = Vec<Vec<u8>>;

fn main() -> Result<()> {
    let input = parse_input(include_str!("../inputs/03/input.txt"))?;

    let part1 = part_01(&input);
    println!("Part 01: {part1}");
    let part2 = part_02(&input);
    println!("Part 02: {part2}");
    Ok(())
}

fn part_01(input: &Input) -> u64 {
    input
        .iter()
        .map(|bank| {
            let it = bank.as_slice()[0..(bank.len() - 1)].iter().copied();
            let (first_index, first_digit) =
                highest(it).expect("There should be at least one item in a bank");
            let it = bank.as_slice()[(first_index + 1)..].iter().copied();
            let (_second_index, second_digit) =
                highest(it).expect("There should be a second item in the bank");
            first_digit as u64 * 10 + second_digit as u64
        })
        .sum()
}

fn part_02(input: &Input) -> u64 {
    const DIGITS: usize = 12;
    input
        .iter()
        .map(|bank| {
            let mut start = 0;
            let mut res = 0;
            for i in 0..DIGITS {
                let digits_left = DIGITS - i;
                let it = bank.as_slice()[start..(bank.len() - (digits_left - 1))]
                    .iter()
                    .copied();
                let (index, digit) =
                    highest(it).expect("There should be at least one item in a bank");
                res *= 10;
                res += digit as u64;
                start += index + 1;
            }
            res
        })
        .sum()
}

fn highest<I>(iter: I) -> Option<(usize, u8)>
where
    I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
{
    // We need to reverse the list because max_by takes the last maximum while we want the first
    iter.enumerate().rev().max_by(|x, y| x.1.cmp(&y.1))
}

fn parse_input(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).map(|d| d as u8))
                .collect::<Option<Vec<u8>>>()
        })
        .collect::<Option<Input>>()
        .ok_or_else(|| anyhow!("Failed to parse"))
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_01, part_02};

    #[test]
    fn max_joltage_01() {
        let input = parse_input("811111111111119").unwrap();
        assert_eq!(89, part_01(&input));
    }

    #[test]
    fn max_joltage_02() {
        let input = parse_input("987654321111111").unwrap();
        assert_eq!(987654321111, part_02(&input));
    }
}
