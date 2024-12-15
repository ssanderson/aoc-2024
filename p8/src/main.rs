use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use common::grid::{Coord, Grid};

fn main() -> anyhow::Result<()> {
    let input: Input = include_str!("input.txt").parse()?;
    println!("p1: {}", p1(&input));
    println!("p1: {}", p2(&input));
    Ok(())
}

fn p1(input: &Input) -> usize {
    let locs = input.antenna_positions();

    // Compute antinodes.
    let mut antinodes: HashSet<Coord> = HashSet::new();
    for coords in locs.values() {
        for (i, first) in coords.iter().enumerate() {
            for second in coords[i + 1..].iter() {
                antinodes.extend(get_antinodes_p1(&input.0, *first, *second))
            }
        }
    }
    antinodes.len()
}

fn p2(input: &Input) -> usize {
    let locs = input.antenna_positions();

    // Compute antinodes.
    let mut antinodes: HashSet<Coord> = HashSet::new();
    for coords in locs.values() {
        for (i, first) in coords.iter().enumerate() {
            for second in coords[i + 1..].iter() {
                antinodes.extend(get_antinodes_p2(&input.0, *first, *second))
            }
        }
    }
    antinodes.len()
}

fn get_antinodes_p1(grid: &Grid<Cell>, a: Coord, b: Coord) -> impl Iterator<Item = Coord> {
    let first = a + (b - a);
    let second = b + (a - b);

    let c1 = if grid.in_bounds(first) {
        Some(first)
    } else {
        None
    };

    let c2 = if grid.in_bounds(second) {
        Some(second)
    } else {
        None
    };

    c1.into_iter().chain(c2)
}

fn get_antinodes_p2(grid: &Grid<Cell>, a: Coord, b: Coord) -> impl Iterator<Item = Coord> + '_ {
    let delta = b - a;

    // Line from a to inf in positive direction.
    let plus = (0..)
        .into_iter()
        .map(move |i| a + (delta * i))
        .take_while(|coord| grid.in_bounds(*coord));

    // Line from a to inf in negative direction.
    let minus = (1..)
        .into_iter()
        .map(move |i| a - (delta * i))
        .take_while(|coord| grid.in_bounds(*coord));

    plus.chain(minus)
}

struct Input(Grid<Cell>);

impl Input {
    fn antenna_positions(&self) -> HashMap<char, Vec<Coord>> {
        let mut locs: HashMap<char, Vec<Coord>> = HashMap::new();
        for (loc, cell) in self.0.iter_cells() {
            match cell {
                Cell::Empty => continue,
                Cell::Antenna(c) => {
                    locs.entry(*c).or_default().push(loc);
                }
            }
        }
        locs
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

enum Cell {
    Empty,
    Antenna(char),
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => Ok(Self::Antenna(c)),
            '.' => Ok(Self::Empty),
            _ => Err(anyhow::anyhow!("invalid cell {c}")),
        }
    }
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_p1() -> anyhow::Result<()> {
        let input = EXAMPLE.parse()?;
        let result = super::p1(&input);
        assert_eq!(result, 14);
        Ok(())
    }

    #[test]
    fn test_p2() -> anyhow::Result<()> {
        let input = EXAMPLE.parse()?;
        let result = super::p2(&input);
        assert_eq!(result, 34);
        Ok(())
    }
}
