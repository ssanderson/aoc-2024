use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt");
    println!("p1: {}", p1(input)?);
    println!("p2: {}", p2(input)?);
    Ok(())
}

pub fn p1(s: &str) -> anyhow::Result<u32> {
    let grid: Grid = s.parse()?;

    let xmas = [Letter::X, Letter::M, Letter::A, Letter::S];

    let mut count = 0;
    for coord in grid.iter_coords() {
        for dir in Delta::directions() {
            if grid.matches(coord, &xmas, dir) {
                count += 1;
            }
        }
    }
    Ok(count)
}

pub fn p2(s: &str) -> anyhow::Result<u32> {
    let grid: Grid = s.parse()?;
    let mut count = 0;
    for coord in grid.iter_coords() {
        if grid.is_center_of_xmas(coord) {
            count += 1;
        }
    }
    Ok(count)
}

struct Grid {
    cells: Vec<Letter>,
    w: usize,
    h: usize,
}

impl Grid {
    pub fn new(cells: Vec<Letter>, w: usize, h: usize) -> anyhow::Result<Self> {
        if cells.len() != h * w {
            anyhow::bail!("{size} != {h} * {w}", size = cells.len());
        }
        Ok(Self { cells, w, h })
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = Coord> {
        let w = self.w;
        let h = self.h;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        std::iter::from_fn(move || {
            if y as usize == h {
                return None;
            }

            let prev = Coord(x, y);

            x += 1;
            if x as usize == w {
                x = 0;
                y += 1;
            }

            Some(prev)
        })
    }

    pub fn at(&self, Coord(x, y): Coord) -> Option<Letter> {
        if x < 0 || (x as usize) >= self.w || y < 0 || (y as usize) >= self.h {
            return None;
        }
        Some(self.cells[x as usize + y as usize * self.w])
    }

    pub fn matches(&self, coord: Coord, suffix: &[Letter], delta: Delta) -> bool {
        let Some((first, rest)) = suffix.split_first() else {
            // found a match
            return true;
        };

        let Some(val) = self.at(coord) else {
            // Outside of grid.
            return false;
        };

        &val == first && self.matches(coord.offset(delta), rest, delta)
    }

    pub fn is_center_of_xmas(&self, coord: Coord) -> bool {
        if self.at(coord) != Some(Letter::A) {
            return false;
        }

        let sam = [Letter::S, Letter::A, Letter::M];
        let mas = [Letter::M, Letter::A, Letter::S];

        for dir in [Delta(-1, -1), Delta(-1, 1)] {
            let offset = coord.offset(dir);
            let inv = dir.invert();
            if !self.matches(offset, &sam, inv) && !self.matches(offset, &mas, inv) {
                return false;
            }
        }

        true
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lengths: Vec<usize> = s.lines().map(|l| l.len()).collect();
        let Some(first) = lengths.first() else {
            anyhow::bail!("no lines in input");
        };
        if lengths.iter().any(|len| len != first) {
            anyhow::bail!("unequal line lengths");
        }

        let h = lengths.len();
        let w = *first;
        let cells = s
            .chars()
            .filter(|&c| c != '\n')
            .map(Letter::try_from)
            .collect::<Result<Vec<Letter>, _>>()?;

        Self::new(cells, w, h)
    }
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

#[derive(Debug, Clone, Copy)]
struct Coord(i32, i32);

impl Coord {
    pub fn offset(self, delta: Delta) -> Self {
        Self(self.0 + delta.0, self.1 + delta.1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Delta(i32, i32);

impl Delta {
    pub fn invert(self) -> Self {
        Self(-self.0, -self.1)
    }

    fn directions() -> impl Iterator<Item = Self> {
        const ALL: [Delta; 8] = [
            Delta(-1, -1),
            Delta(0, -1),
            Delta(1, -1),
            Delta(-1, 0),
            Delta(1, 0),
            Delta(-1, 1),
            Delta(0, 1),
            Delta(1, 1),
        ];
        ALL.iter().copied()
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
