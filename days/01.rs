use anyhow::anyhow;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../inputs/01/input.txt");
    let input = parse_input(input)?;
    let part1 = part_one(&input);
    println!("Part one: {}", part1);
    let part2 = part_two(&input);
    println!("Part one: {}", part2);
    Ok(())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Rotation>> {
    input.lines().map(|l| l.parse::<Rotation>()).collect()
}

fn part_one(input: &[Rotation]) -> usize {
    input
        .iter()
        .fold((50, 0), |(pos, zeroes), rotation| {
            let pos = rotation.rotate(pos);
            let zeroes = if pos == 0 { zeroes + 1 } else { zeroes };
            (pos, zeroes)
        })
        .1
}

fn part_two(input: &[Rotation]) -> usize {
    input
        .iter()
        .fold((50, 0), |(mut pos, mut zeroes), rotation| {
            let mut result = (pos, zeroes);
            let (clicks, rotation) = rotation.ones();
            for _i in 0..clicks {
                pos = rotation.rotate(pos);
                zeroes = if pos == 0 { zeroes + 1 } else { zeroes };
                result = (pos, zeroes);
            }
            result
        })
        .1
}

enum Rotation {
    Left(usize),
    Right(usize),
}

impl Rotation {
    fn rotate(&self, from: usize) -> usize {
        match self {
            Self::Left(clicks) => (100 + from - (clicks % 100)) % 100,
            Self::Right(clicks) => (from + (clicks % 100)) % 100,
        }
    }

    fn ones(&self) -> (usize, Self) {
        match self {
            Self::Left(clicks) => (*clicks, Self::Left(1)),
            Self::Right(clicks) => (*clicks, Self::Right(1)),
        }
    }
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix('L') {
            return Ok(Self::Left(rest.parse::<usize>()?));
        }

        if let Some(rest) = s.strip_prefix('R') {
            return Ok(Self::Right(rest.parse::<usize>()?));
        }

        Err(anyhow!("Can't parse {s}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rotation() {
        assert_eq!(1, Rotation::Right(1).rotate(0));
        assert_eq!(0, Rotation::Right(1).rotate(99));

        assert_eq!(0, Rotation::Left(1).rotate(1));
        assert_eq!(99, Rotation::Left(1).rotate(0));
    }
}
