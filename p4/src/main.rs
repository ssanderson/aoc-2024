use common::grid::{Coord, Delta, Grid};

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt");
    println!("p1: {}", p1(input)?);
    println!("p2: {}", p2(input)?);
    Ok(())
}

pub fn p1(s: &str) -> anyhow::Result<u32> {
    let grid: Grid<Letter> = s.parse()?;

    let xmas = [Letter::X, Letter::M, Letter::A, Letter::S];

    let mut count = 0;
    for coord in grid.iter_coords() {
        for dir in Delta::directions() {
            if matches(&grid, coord, &xmas, dir) {
                count += 1;
            }
        }
    }
    Ok(count)
}

pub fn p2(s: &str) -> anyhow::Result<u32> {
    let grid: Grid<Letter> = s.parse()?;
    let mut count = 0;
    for coord in grid.iter_coords() {
        if is_center_of_xmas(&grid, coord) {
            count += 1;
        }
    }
    Ok(count)
}

type MyGrid = Grid<Letter>;

fn matches(grid: &MyGrid, coord: Coord, suffix: &[Letter], delta: Delta) -> bool {
    let Some((first, rest)) = suffix.split_first() else {
        // found a match
        return true;
    };

    let Some(val) = grid.at(coord) else {
        // Outside of grid.
        return false;
    };

    val == first && matches(grid, coord.offset(delta), rest, delta)
}

fn is_center_of_xmas(grid: &MyGrid, coord: Coord) -> bool {
    if grid.at(coord) != Some(&Letter::A) {
        return false;
    }

    let sam = [Letter::S, Letter::A, Letter::M];
    let mas = [Letter::M, Letter::A, Letter::S];

    for dir in [Delta(-1, -1), Delta(-1, 1)] {
        let offset = coord.offset(dir);
        let inv = dir.invert();
        if !matches(&grid, offset, &sam, inv) && !matches(&grid, offset, &mas, inv) {
            return false;
        }
    }

    true
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Letter {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::X),
            'M' => Ok(Self::M),
            'A' => Ok(Self::A),
            'S' => Ok(Self::S),
            c => Err(anyhow::anyhow!("bad letter: {c}")),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_p1() -> anyhow::Result<()> {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let result = super::p1(input)?;
        assert_eq!(result, 18);
        Ok(())
    }

    #[test]
    fn test_p2() -> anyhow::Result<()> {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let result = super::p2(input)?;
        assert_eq!(result, 9);
        Ok(())
    }
}
