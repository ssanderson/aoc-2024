use std::{collections::HashSet, str::FromStr};

use common::grid::{CardinalDirection, Coord, Grid};

fn main() -> anyhow::Result<()> {
    let problem: Map = include_str!("input.txt").parse()?;
    // println!("p1: {}", p1(&problem));
    println!("p2: {}", p2(&problem));
    Ok(())
}

fn p1(m: &Map) -> usize {
    let (trace, _) = m.simulate();
    trace
        .iter()
        .map(|g| g.position)
        .collect::<HashSet<_>>()
        .len()
}

fn p2(m: &Map) -> usize {
    let mut count = 0;
    for (pos, cell) in m.grid.iter_cells() {
        if pos == m.guard.position || *cell != MapCell::Empty {
            continue;
        }

        let mut cloned = m.clone();
        *cloned.grid.at_mut(pos).unwrap() = MapCell::Wall;
        let (_, endcond) = cloned.simulate();
        if endcond == EndCondition::Loop {
            count += 1
        }

        coz::progress!();
    }
    count
}

#[derive(Clone)]
struct Map {
    pub grid: Grid<MapCell>,
    pub guard: Guard,
}

impl Map {
    fn simulate(&self) -> (HashSet<Guard>, EndCondition) {
        let mut trace = HashSet::new();
        trace.insert(self.guard.clone());

        let mut guard = self.guard.clone();
        loop {
            let next = guard.position.offset(guard.direction.into());
            match self.grid.at(next) {
                None => return (trace, EndCondition::OffMap),
                Some(MapCell::Empty) => {
                    guard.position = next;
                }
                Some(MapCell::Wall) => {
                    guard.direction = guard.direction.rotate_clockwise();
                }
            }
            if !trace.insert(guard.clone()) {
                return (trace, EndCondition::Loop);
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum EndCondition {
    OffMap,
    Loop,
}

impl FromStr for Map {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<ParseCell> = s.parse()?;
        let guard = (|| {
            for (coord, cell) in grid.iter_cells() {
                if let ParseCell::Guard(d) = cell {
                    return Ok(Guard {
                        position: coord,
                        direction: *d,
                    });
                }
            }
            anyhow::bail!("no person in grid")
        })()?;

        let grid = grid.map(|c| match c {
            ParseCell::Empty => MapCell::Empty,
            ParseCell::Wall => MapCell::Wall,
            ParseCell::Guard(_) => MapCell::Empty,
        });

        Ok(Map { grid, guard })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapCell {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum ParseCell {
    Empty,
    Wall,
    Guard(CardinalDirection),
}

impl TryFrom<char> for ParseCell {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(ParseCell::Empty),
            '#' => Ok(ParseCell::Wall),
            '^' => Ok(ParseCell::Guard(CardinalDirection::Up)),
            '>' => Ok(ParseCell::Guard(CardinalDirection::Right)),
            '<' => Ok(ParseCell::Guard(CardinalDirection::Left)),
            'v' => Ok(ParseCell::Guard(CardinalDirection::Down)),
            c => anyhow::bail!("invalid character: {c}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    position: Coord,
    direction: CardinalDirection,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_p1() -> anyhow::Result<()> {
        let input: super::Map = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
            .parse()?;

        assert_eq!(super::p1(&input), 41);

        Ok(())
    }

    #[test]
    fn test_p2() -> anyhow::Result<()> {
        let input: super::Map = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
            .parse()?;

        assert_eq!(super::p2(&input), 6);

        Ok(())
    }
}
