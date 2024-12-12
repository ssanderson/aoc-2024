use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let problem: Problem = include_str!("input.txt").parse()?;
    println!("p1: {}", problem.p1());
    println!("p1: {}", problem.p2());
    Ok(())
}

struct Problem {
    equations: Vec<Equation>,
}

impl Problem {
    fn p1(&self) -> i64 {
        self.equations
            .iter()
            .filter_map(|eq| {
                if eq.is_solvable_add_mul() {
                    Some(eq.target)
                } else {
                    None
                }
            })
            .sum()
    }

    fn p2(&self) -> i64 {
        self.equations
            .iter()
            .filter_map(|eq| {
                if eq.is_solvable_add_mul_concat() {
                    Some(eq.target)
                } else {
                    None
                }
            })
            .sum()
    }
}

impl FromStr for Problem {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let equations: Vec<Equation> = s
            .lines()
            .map(Equation::from_str)
            .collect::<Result<_, _>>()?;
        Ok(Self { equations })
    }
}

struct Equation {
    target: i64,
    values: Vec<i64>,
}

impl Equation {
    fn is_solvable_add_mul(&self) -> bool {
        let mut targets = vec![self.target];
        let mut vals = self.values.clone();

        while let Some(next) = vals.pop() {
            targets = targets
                .into_iter()
                .flat_map(|target| {
                    if target < next {
                        return Vec::from([]).into_iter();
                    }
                    if target % next == 0 {
                        return Vec::from([target - next, target / next]).into_iter();
                    } else {
                        return Vec::from([target - next]).into_iter();
                    }
                })
                .collect();
        }
        targets.contains(&0)
    }

    fn is_solvable_add_mul_concat(&self) -> bool {
        let mut targets = vec![self.target];
        let mut vals = self.values.clone();

        while let Some(next) = vals.pop() {
            targets = targets
                .into_iter()
                .flat_map(|target| {
                    if target < next {
                        return Vec::new().into_iter();
                    }

                    let mut nums = vec![target - next];

                    if target % next == 0 {
                        nums.push(target / next)
                    }

                    if target.to_string().ends_with(&next.to_string()) {
                        nums.push(target / (10_i64.pow(next.ilog10() + 1)));
                    }

                    nums.into_iter()
                })
                .collect();
        }
        targets.contains(&0)
    }
}

impl FromStr for Equation {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(": ").collect();
        if parts.len() != 2 {
            anyhow::bail!("failed to parse equation: {s}");
        }
        let target: i64 = parts[0].parse()?;
        let values: Vec<i64> = parts[1]
            .split(" ")
            .map(i64::from_str)
            .collect::<Result<_, _>>()?;
        Ok(Self { target, values })
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_p1() -> anyhow::Result<()> {
        let input: super::Problem = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#
            .parse()?;

        assert_eq!(input.p1(), 3749);

        Ok(())
    }

    #[test]
    fn test_p2() -> anyhow::Result<()> {
        let input: super::Problem = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#
            .parse()?;

        assert_eq!(input.p2(), 11387);

        Ok(())
    }
}
