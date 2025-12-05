use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

struct Equation {
    nums: Vec<u64>,
    op: Op,
}

impl Equation {
    fn compute(&self) -> u64 {
        match self.op {
            Op::Add => self.nums.iter().sum(),
            Op::Mul => self.nums.iter().product(),
        }
    }
}

type Input = Vec<Equation>;

fn main() -> Result<()> {
    let raw_input = include_str!("../inputs/06/input.txt");
    let input = parse_input(raw_input)?;

    let part1 = part_01(&input);
    println!("Part 01: {part1}");
    let part2 = part_02(raw_input);
    println!("Part 02: {part2}");
    Ok(())
}

fn part_01(input: &Input) -> u64 {
    input.iter().map(|eq| eq.compute()).sum()
}

fn part_02(input: &str) -> u64 {
    let mat: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut total = 0;

    let rows = mat.len();
    let cols = mat.iter().map(|r| r.len()).max().unwrap();

    let mut buf: Vec<u64> = vec![];
    let mut next_op = Op::Add;

    for j in 0..cols {
        let mut next_num = 0;
        let data = mat.iter().flat_map(|row| row.get(j)).collect::<Vec<_>>();

        if data.iter().all(|ch| ch.is_whitespace()) {
            if next_op == Op::Add {
                total += buf.iter().sum::<u64>();
            } else {
                total += buf.iter().product::<u64>();
            }

            buf.clear();

            continue;
        }

        for ch in data {
            match *ch {
                '+' => next_op = Op::Add,
                '*' => next_op = Op::Mul,
                '0'..='9' => {
                    next_num *= 10;
                    next_num += ch.to_digit(10).unwrap() as u64;
                }
                _ => {}
            }
        }

        buf.push(next_num);
    }

    if next_op == Op::Add {
        total += buf.iter().sum::<u64>();
    } else {
        total += buf.iter().product::<u64>();
    }

    total
}

fn parse_input(input: &str) -> Result<Input> {
    let mut nums: Vec<Vec<u64>> = vec![];
    let mut ops: Vec<Op> = vec![];
    for line in input.lines() {
        if line.starts_with('*') || line.starts_with('+') {
            ops = line
                .split_whitespace()
                .map(|item| match item {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    _ => unreachable!(),
                })
                .collect();
            break;
        }

        nums.push(
            line.split_whitespace()
                .map(|n| Ok(n.parse::<u64>()?))
                .collect::<Result<_>>()?,
        );
    }

    let mut equations = vec![];
    for i in 0..nums[0].len() {
        equations.push(Equation {
            nums: nums.iter().map(|row| row[i]).collect(),
            op: ops[i],
        });
    }

    Ok(equations)
}
