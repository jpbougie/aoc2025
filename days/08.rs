use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Box {
    x: i64,
    y: i64,
    z: i64,
}

impl Box {
    fn sq_dist(&self, other: &Self) -> i64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        x * x + y * y + z * z
    }
}

impl FromStr for Box {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split(',');
        Ok(Self {
            x: parts.next().unwrap().parse()?,
            y: parts.next().unwrap().parse()?,
            z: parts.next().unwrap().parse()?,
        })
    }
}

type Input = Vec<Box>;

type NodeId = usize;

fn main() -> Result<()> {
    let input = parse_input(include_str!("../inputs/08/input.txt"))?;
    let part1 = part_01(&input);
    println!("Part 01: {part1}");
    let part2 = part_02(&input);
    println!("Part 02: {part2}");
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Dist {
    n1: NodeId,
    n2: NodeId,
    dist: i64,
}

impl Ord for Dist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Dist {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}

fn part_01(input: &Input) -> usize {
    const CONNECTIONS: usize = 1000;
    let mut circuits: Vec<HashSet<NodeId>> = vec![];
    let mut nodes_to_circuits = HashMap::with_capacity(input.len());
    let mut distances = BinaryHeap::with_capacity(input.len() * input.len() / 2);
    for (id1, b1) in input.iter().enumerate() {
        for (id2, b2) in (&input[(id1 + 1)..]).iter().enumerate() {
            distances.push(Dist {
                n1: id1,
                n2: id1 + 1 + id2,
                dist: b1.sq_dist(b2),
            });
        }
    }

    for _i in 0..CONNECTIONS {
        let Some(Dist { n1, n2, dist: _ }) = distances.pop() else {
            break;
        };

        merge_circuits(n1, n2, &mut circuits, &mut nodes_to_circuits);
    }

    let mut sizes = circuits
        .iter()
        .filter_map(|c| if c.is_empty() { None } else { Some(c.len()) })
        .collect::<Vec<usize>>();

    sizes.sort();

    let max = &sizes[sizes.len() - 3..];

    max.iter().product()
}

type CircuitId = usize;
fn merge_circuits(
    n1: NodeId,
    n2: NodeId,
    circuits: &mut Vec<HashSet<NodeId>>,
    nodes_to_circuits: &mut HashMap<NodeId, CircuitId>,
) -> CircuitId {
    if nodes_to_circuits.get(&n1).is_none() && nodes_to_circuits.get(&n2).is_none() {
        // new circuit
        let circuit_id = circuits.len();
        let mut set = HashSet::new();
        set.insert(n1);
        set.insert(n2);
        circuits.push(set);

        nodes_to_circuits.insert(n1, circuit_id);
        nodes_to_circuits.insert(n2, circuit_id);

        return circuit_id;
    }
    if let Some(c1) = nodes_to_circuits.get(&n1).copied()
        && nodes_to_circuits.get(&n2).is_none()
    {
        circuits[c1].insert(n2);
        nodes_to_circuits.insert(n2, c1);
        return c1;
    }

    if let Some(c2) = nodes_to_circuits.get(&n2).copied()
        && !nodes_to_circuits.contains_key(&n1)
    {
        circuits[c2].insert(n1);
        nodes_to_circuits.insert(n1, c2);
        return c2;
    }

    let base = *nodes_to_circuits.get(&n1).unwrap();
    let to_merge = *nodes_to_circuits.get(&n2).unwrap();

    if base == to_merge {
        return base;
    }

    let cc = circuits[to_merge].clone();
    for n in cc {
        circuits[base].insert(n);
        nodes_to_circuits.insert(n, base);
    }
    circuits[to_merge].clear();

    return base;
}

fn part_02(input: &Input) -> i64 {
    let mut circuits: Vec<HashSet<NodeId>> = vec![];
    let mut nodes_to_circuits = HashMap::with_capacity(input.len());
    let mut distances = BinaryHeap::with_capacity(input.len() * input.len() / 2);
    for (id1, b1) in input.iter().enumerate() {
        for (id2, b2) in (&input[(id1 + 1)..]).iter().enumerate() {
            distances.push(Dist {
                n1: id1,
                n2: id1 + 1 + id2,
                dist: b1.sq_dist(b2),
            });
        }
    }

    loop {
        let Some(Dist { n1, n2, dist: _ }) = distances.pop() else {
            break;
        };

        let c = merge_circuits(n1, n2, &mut circuits, &mut nodes_to_circuits);
        if circuits[c].len() == input.len() {
            return input[n1].x * input[n2].x;
        }
    }

    return 0;
}

fn parse_input(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|l| l.parse::<Box>())
        .collect::<Result<Vec<_>>>()
}
