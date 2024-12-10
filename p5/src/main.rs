use std::{cmp::Ordering, collections::HashSet, str::FromStr};

fn main() -> anyhow::Result<()> {
    let problem: Problem = include_str!("input.txt").parse()?;
    println!("p1: {}", problem.p1());
    println!("p2: {}", problem.p2());
    Ok(())
}

#[derive(Debug)]
struct Problem {
    edges: HashSet<(u32, u32)>,
    lists: Vec<Vec<u32>>,
}

impl Problem {
    fn p1(&self) -> u32 {
        self.lists
            .iter()
            .filter_map(|list| {
                if self.satisfies_constraints(&list) {
                    Some(list[list.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }

    fn p2(&self) -> u32 {
        self.lists
            .iter()
            .filter_map(|list| {
                if self.satisfies_constraints(&list) {
                    return None;
                }

                let mut list = list.clone();
                list.sort_by(|&a, &b| {
                    if self.edges.contains(&(a, b)) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });

                Some(list[list.len() / 2])
            })
            .sum()
    }

    fn satisfies_constraints(&self, list: &[u32]) -> bool {
        let size = list.len();
        for i in 0..size {
            for j in i + 1..size {
                if self.edges.contains(&(list[j], list[i])) {
                    return false;
                }
            }
        }
        true
    }
}

impl FromStr for Problem {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let edges: HashSet<(u32, u32)> = lines
            .clone()
            .take_while(|&s| s != "")
            .map(|s| -> anyhow::Result<(u32, u32)> {
                let mut parts = s.split('|');
                let Some(first) = parts.next() else {
                    anyhow::bail!("failed to parse line: {s}");
                };
                let Some(second) = parts.next() else {
                    anyhow::bail!("failed to parse line: {s}");
                };
                Ok((first.parse()?, second.parse()?))
            })
            .collect::<Result<_, _>>()?;

        let lists = lines
            .skip_while(|&s| s != "")
            .skip(1)
            .map(|s| -> anyhow::Result<Vec<u32>> {
                Ok(s.split(",").map(u32::from_str).collect::<Result<_, _>>()?)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { edges, lists })
    }
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    pub fn test_p1() -> anyhow::Result<()> {
        let problem: super::Problem = EXAMPLE.parse()?;
        assert_eq!(problem.p1(), 143);
        Ok(())
    }

    #[test]
    pub fn test_p2() -> anyhow::Result<()> {
        let problem: super::Problem = EXAMPLE.parse()?;
        assert_eq!(problem.p2(), 123);
        Ok(())
    }
}
