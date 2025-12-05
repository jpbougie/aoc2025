use std::ops::RangeInclusive;

use anyhow::{Result, anyhow};

struct Input {
    ranges: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

fn main() -> Result<()> {
    let input = parse_input(include_str!("../inputs/05/input.txt"))?;

    let part1 = part_01(&input);
    println!("Part 01: {part1}");
    let part2 = part_02(&input);
    println!("Part 02: {part2}");
    Ok(())
}

fn part_01(input: &Input) -> usize {
    input
        .ingredients
        .iter()
        .filter(|ing| input.ranges.iter().any(|r| r.contains(ing)))
        .count()
}

fn part_02(input: &Input) -> u64 {
    let mut ranges = input.ranges.clone();
    ranges.sort_by_key(|r| r.start().clone());

    let mut total = 0;
    let mut it = ranges.iter().peekable();
    'outer: loop {
        let Some(cur) = it.next() else {
            break;
        };
        loop {
            let Some(next) = it.peek() else {
                total += *cur.end() - *cur.start() + 1;
                break;
            };

            if next.start() > cur.end() {
                total += *cur.end() - *cur.start() + 1;
                continue 'outer;
            }

            // Totally embedded, we can ignore it
            if next.end() < cur.end() {
                it.next();
                continue;
            }

            // Overlaps but extends, only add the part
            total += next.start() - cur.start();
            continue 'outer;
        }
    }

    total
}

fn parse_input(input: &str) -> Result<Input> {
    let Some((rgs, ings)) = input.split_once("\n\n") else {
        return Err(anyhow!("No parts split"));
    };

    let ranges = rgs
        .lines()
        .map(|line| {
            let Some((from, to)) = line.split_once('-') else {
                return Err(anyhow!("No - separator"));
            };
            let from = from.parse::<u64>()?;
            let to = to.parse::<u64>()?;

            Ok(from..=to)
        })
        .collect::<Result<Vec<RangeInclusive<u64>>>>()?;

    let ingredients = ings
        .lines()
        .map(|l| Ok(l.parse::<u64>()?))
        .collect::<Result<Vec<u64>>>()?;

    Ok(Input {
        ranges,
        ingredients,
    })
}
