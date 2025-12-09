use anyhow::{Result, anyhow};

type Input = Vec<(i64, i64)>;

fn main() -> Result<()> {
    let input = parse_input(include_str!("../inputs/09/input.txt"))?;

    let part1 = part_01(&input);
    println!("Part 01: {part1}");
    let part2 = part_02(&input);
    println!("Part 02: {part2}");
    Ok(())
}

fn parse_input(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|l| {
            let Some((a, b)) = l.split_once(',') else {
                return Err(anyhow!("No comma"));
            };

            Ok((a.parse()?, b.parse()?))
        })
        .collect::<Result<Input>>()
}

fn part_01(input: &Input) -> u64 {
    let mut max_size = 0;
    for (i, x) in input.iter().enumerate() {
        for y in &input[i + 1..] {
            let a = area(x, y);
            if a > max_size {
                max_size = a;
            }
        }
    }

    max_size
}

fn area(a: &(i64, i64), b: &(i64, i64)) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

#[derive(Debug)]
struct Rect {
    a: (i64, i64),
    b: (i64, i64),
}

impl Rect {
    fn corners(&self) -> Vec<(i64, i64)> {
        vec![self.a, self.b, (self.a.0, self.b.1), (self.b.0, self.a.1)]
    }

    fn edges(&self) -> Vec<Segment> {
        vec![
            Segment::new(self.a, (self.a.0, self.b.1)),
            Segment::new((self.a.0, self.b.1), self.b),
            Segment::new(self.b, (self.b.0, self.a.1)),
            Segment::new((self.b.0, self.a.1), self.a),
        ]
    }
}

fn part_02(input: &Input) -> u64 {
    let mut max_size = 0;

    let mut edges = input
        .windows(2)
        .map(|e| e.to_vec())
        .collect::<Vec<Vec<_>>>();

    edges.push(vec![input[input.len() - 1], input[0]]);

    let edges = edges;

    let segments = edges
        .iter()
        .map(|edge| Segment::new(edge[0], edge[1]))
        .collect::<Vec<_>>();

    for (i, x) in input.iter().enumerate() {
        for y in &input[i + 1..] {
            let r = Rect { a: *x, b: *y };
            let a = area(x, y);
            if a <= max_size {
                continue;
            }

            if !r.corners().into_iter().all(|corner| {
                input.iter().any(|p| *p == corner) || point_in_polygon(&edges, corner)
            }) {
                continue;
            }

            if r.edges()
                .into_iter()
                .any(|corner| segments.iter().any(|segment| corner.collides_with(segment)))
            {
                continue;
            }

            max_size = a;
        }
    }

    max_size
}

fn point_in_polygon(edges: &[Vec<(i64, i64)>], point: (i64, i64)) -> bool {
    // Count the number of pairs of vertical edges (where y1=y2) where y1 > y && x >= x1 && x <= x2
    // odd means inside
    let collisions = edges
        .iter()
        .filter(|edges| {
            let (x1, x2) = if edges[0].0 < edges[1].0 {
                (edges[0].0, edges[1].0)
            } else {
                (edges[1].0, edges[0].0)
            };
            edges[0].1 == edges[1].1 && edges[0].1 > point.1 && point.0 > x1 && point.0 <= x2
        })
        .count();

    collisions % 2 != 0
}

struct Segment {
    a: (i64, i64),
    b: (i64, i64),
}

impl Segment {
    fn new(a: (i64, i64), b: (i64, i64)) -> Self {
        if a.0 == b.0 {
            let (miny, maxy) = if a.1 <= b.1 { (a.1, b.1) } else { (b.1, a.1) };
            Self {
                a: (a.0, miny),
                b: (a.0, maxy),
            }
        } else {
            let (minx, maxx) = if a.0 <= b.0 { (a.0, b.0) } else { (b.0, a.0) };
            Self {
                a: (minx, a.1),
                b: (maxx, a.1),
            }
        }
    }
    fn orientation(&self) -> Orientation {
        if self.a.0 == self.b.0 {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        }
    }

    fn collides_with(&self, other: &Self) -> bool {
        let orientation = self.orientation();
        let other_orientation = other.orientation();
        if orientation == other_orientation {
            return false;
        }

        let (h, v) = if orientation == Orientation::Horizontal {
            (self, other)
        } else {
            (other, self)
        };

        h.a.0 > v.a.0 && h.a.0 < v.b.0 && h.a.1 < v.a.1 && h.b.1 > v.a.1
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[cfg(test)]
mod tests {
    use crate::area;

    #[test]
    fn test_area() {
        assert_eq!(50, area(&(2, 5), &(11, 1)));
    }
}
