use std::str::FromStr;

pub struct Grid<Cell> {
    cells: Vec<Cell>,
    w: usize,
    h: usize,
}

impl<T> Grid<T> {
    pub fn new(cells: Vec<T>, w: usize, h: usize) -> anyhow::Result<Self> {
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

    pub fn at(&self, Coord(x, y): Coord) -> Option<&T> {
        if x < 0 || (x as usize) >= self.w || y < 0 || (y as usize) >= self.h {
            return None;
        }
        Some(&self.cells[x as usize + y as usize * self.w])
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

#[derive(Debug, Clone, Copy)]
pub struct Coord(pub i32, pub i32);

impl Coord {
    pub fn offset(self, delta: Delta) -> Self {
        Self(self.0 + delta.0, self.1 + delta.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Delta(pub i32, pub i32);

impl Delta {
    pub fn invert(self) -> Self {
        Self(-self.0, -self.1)
    }

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
