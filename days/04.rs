use anyhow::{Result, anyhow};
use grid::{Cell, Grid};

type Input = Grid<Stuff>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Stuff {
    Empty,
    Roll,
}

fn main() -> Result<()> {
    let input = parse_input(include_str!("../inputs/04/input.txt"))?;

    let part1 = part_01(&input);
    println!("Part 01: {part1}");
    let part2 = part_02(input.clone());
    println!("Part 02: {part2}");
    Ok(())
}

fn removable(grid: &Grid<Stuff>, cell: &Cell<Stuff>) -> bool {
    grid.neighbours(cell.row, cell.col)
        .iter()
        .filter(|(rr, cc)| matches!(grid.get(*rr, *cc).unwrap().val, &Stuff::Roll))
        .count()
        < 4
}

fn part_01(input: &Input) -> usize {
    let mut count = 0;
    for cell in input.iter_cells() {
        if !matches!(cell.val, &Stuff::Roll) {
            continue;
        }

        if removable(input, &cell) {
            count += 1;
        }
    }

    count
}

fn part_02(mut input: Input) -> usize {
    let mut count = 0;
    loop {
        let to_remove = input
            .iter_cells()
            .filter_map(|cell| {
                if cell.val == &Stuff::Roll && removable(&input, &cell) {
                    Some((cell.row, cell.col))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if to_remove.is_empty() {
            break;
        }
        count += to_remove.len();

        for (row, col) in to_remove {
            input.set(row, col, Stuff::Empty);
        }
    }
    count
}

fn parse_input(input: &str) -> Result<Input> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.add_row(
            line.chars()
                .map(|ch| match ch {
                    '.' => Ok(Stuff::Empty),
                    '@' => Ok(Stuff::Roll),
                    _ => Err(anyhow!("Unknown cell {ch}")),
                })
                .collect::<Result<Vec<Stuff>>>()?,
        );
    }

    Ok(grid)
}
