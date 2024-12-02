use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt");
    let parsed: Input = input.parse()?;
    println!("{}", parsed.part1());
    println!("{}", parsed.part2());

    Ok(())
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<Line>,
}

impl Input {
    pub fn part1(&self) -> u32 {
        self.lines.iter().map(|l| u32::from(l.is_safe())).sum()
    }

    pub fn part2(&self) -> u32 {
        self.lines
            .iter()
            .map(|l| u32::from(l.is_safe_with_dampener()))
            .sum()
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { lines })
    }
}

#[derive(Debug)]
struct Line(Vec<u32>);

impl Line {
    fn is_safe(&self) -> bool {
        Self::is_safe_inner(self.0.iter())
    }

    fn is_safe_with_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        // brute force: try to see if the line is safe with each element
        // removed.
        for i in 0..self.0.len() {
            let mut clone = self.0.clone();
            clone.remove(i);
            if Self::is_safe_inner(clone.iter()) {
                return true;
            }
        }

        false
    }

    fn is_safe_inner<'a>(it: impl Iterator<Item = &'a u32> + Clone) -> bool {
        let mut it = std::iter::zip(it.clone(), it.skip(1)).peekable();

        let is_increasing = match it.peek() {
            Some((a, b)) => a < b,
            None => return false,
        };

        let is_unsafe = |a: u32, b: u32| {
            let diff = a.abs_diff(b);
            diff == 0 || diff > 3 || (a < b) != is_increasing
        };

        it.find(|(&a, &b)| is_unsafe(a, b)).is_none()
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(' ').map(str::parse).collect::<Result<_, _>>()?,
        ))
    }
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test_part1() -> anyhow::Result<()> {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let parsed = input.parse::<super::Input>()?;
        let result = parsed.part1();
        assert_eq!(result, 2);

        Ok(())
    }

    #[test]
    pub fn test_part2() -> anyhow::Result<()> {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let parsed = input.parse::<super::Input>()?;
        let result = parsed.part2();
        assert_eq!(result, 4);

        Ok(())
    }
}
