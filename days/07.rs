use std::collections::{HashMap, HashSet};

use anyhow::Result;
use grid::Grid;
use grid_derive::Cellable;

type Input = Grid<Stuff>;

#[derive(Cellable, PartialEq, Eq, Debug)]
enum Stuff {
    #[token('.')]
    Empty,
    #[token('S')]
    Entrance,
    #[token('^')]
    Splitter,
}

fn main() -> Result<()> {
    let input = include_str!("../inputs/07/input.txt").parse::<Grid<Stuff>>()?;

    let part1 = part_01(&input);
    println!("Part 01: {part1}");
    let part2 = part_02(&input);
    println!("Part 02: {part2}");
    Ok(())
}

fn part_01(input: &Input) -> u64 {
    let mut heads = input
        .iter_cells()
        .filter_map(|c| {
            if *c.val == Stuff::Entrance {
                Some((c.row, c.col))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();

    let mut splits = 0;

    while !heads.is_empty() {
        let mut new_heads = Vec::new();
        for head in heads.drain(..) {
            if visited.contains(&head) {
                continue;
            }

            let Some(c) = input.get(head.0 + 1, head.1) else {
                continue;
            };

            if *c.val == Stuff::Splitter {
                splits += 1;
                if c.col > 0 {
                    new_heads.push((c.row, c.col - 1));
                }

                if c.col < input.col_count() - 1 {
                    new_heads.push((c.row, c.col + 1));
                }
            } else {
                new_heads.push((c.row, c.col));
            }

            visited.insert(head);
        }
        heads = new_heads;
    }

    splits
}

fn splits(
    point: (usize, usize),
    grid: &Grid<Stuff>,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(splits) = cache.get(&point) {
        return *splits;
    }

    let Some(c) = grid.get(point.0, point.1) else {
        return 1;
    };

    let val = if *c.val == Stuff::Splitter {
        splits((point.0, point.1 - 1), grid, cache) + splits((point.0, point.1 + 1), grid, cache)
    } else {
        splits((point.0 + 1, point.1), grid, cache)
    };
    cache.insert(point, val);

    val
}

fn part_02(input: &Input) -> usize {
    let entrance = input
        .iter_cells()
        .find(|c| *c.val == Stuff::Entrance)
        .unwrap();
    let mut cache = HashMap::new();
    splits((entrance.row, entrance.col), input, &mut cache)
}

#[cfg(test)]
mod tests {
    use crate::Stuff;

    #[test]
    fn test_parse() {
        let tok: Stuff = '.'.try_into().unwrap();
        assert_eq!(Stuff::Empty, tok);

        let tok: Result<Stuff, _> = 'X'.try_into();
        assert!(tok.is_err());
    }
}
