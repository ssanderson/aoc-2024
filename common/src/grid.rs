use std::str::FromStr;

#[derive(Clone)]
pub struct Grid<Cell> {
    cells: Vec<Cell>,
    pub w: usize,
    pub h: usize,
}

impl<T> Grid<T> {
    pub fn new(cells: Vec<T>, w: usize, h: usize) -> anyhow::Result<Self> {
        if cells.len() != h * w {
            anyhow::bail!("{size} != {h} * {w}", size = cells.len());
        }
        Ok(Self { cells, w, h })
    }

    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Grid<U> {
        Grid {
            cells: self.cells.into_iter().map(f).collect(),
            w: self.w,
            h: self.h,
        }
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (Coord, &T)> {
        self.iter_coords().map(|c| {
            let val = self.at(c).unwrap();
            (c, val)
        })
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

    pub fn in_bounds(&self, Coord(x, y): Coord) -> bool {
        x >= 0 && (x as usize) < self.w && y >= 0 && (y as usize) < self.h
    }

    pub fn at(&self, c @ Coord(x, y): Coord) -> Option<&T> {
        if !self.in_bounds(c) {
            return None;
        }
        Some(&self.cells[x as usize + y as usize * self.w])
    }

    pub fn at_mut(&mut self, c @ Coord(x, y): Coord) -> Option<&mut T> {
        if !self.in_bounds(c) {
            return None;
        }
        Some(&mut self.cells[x as usize + y as usize * self.w])
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<char>,
    anyhow::Error: From<<T as TryFrom<char>>::Error>,
{
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
            .map(T::try_from)
            .collect::<Result<Vec<T>, _>>()?;

        Self::new(cells, w, h)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coord(pub i32, pub i32);

impl std::ops::Add<Delta> for Coord {
    type Output = Coord;

    fn add(self, rhs: Delta) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub<Delta> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Delta) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::Sub<Coord> for Coord {
    type Output = Delta;

    fn sub(self, rhs: Coord) -> Self::Output {
        Delta(rhs.0 - self.0, rhs.1 - self.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Delta(pub i32, pub i32);
impl std::ops::Mul<i32> for Delta {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::Neg for Delta {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl Delta {
    pub fn directions() -> impl Iterator<Item = Self> {
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CardinalDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CardinalDirection {
    pub fn rotate_clockwise(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl Into<Delta> for CardinalDirection {
    fn into(self) -> Delta {
        match self {
            Self::Up => Delta(0, -1),
            Self::Down => Delta(0, 1),
            Self::Left => Delta(-1, 0),
            Self::Right => Delta(1, 0),
        }
    }
}
