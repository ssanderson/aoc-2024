use std::collections::HashMap;

mod grammar;
mod input;

fn main() -> anyhow::Result<()> {
    let input = grammar::InputParser::new().parse(include_str!("input.txt"))?;
    let res1 = part1(input.clone())?;
    println!("{res1}");

    let res1 = part2(input.clone())?;
    println!("{res1}");

    Ok(())
}

fn part1(input: input::Input) -> anyhow::Result<u32> {
    let (mut left, mut right) = input.split();
    left.sort();
    right.sort();

    let res = std::iter::zip(left, right).fold(0, |acc, (a, b)| acc + a.abs_diff(b));
    Ok(res)
}

fn part2(input: input::Input) -> anyhow::Result<u32> {
    let (left, right) = input.split();

    let counts = {
        let mut m = HashMap::new();
        for n in right {
            let entry: &mut u32 = m.entry(n).or_default();
            *entry = *entry + 1;
        }
        m
    };

    let res = left.iter().fold(0, |acc, n| {
        acc + n * counts.get(n).copied().unwrap_or_default()
    });

    Ok(res)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let input = super::grammar::InputParser::new().parse(
            r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#,
        )?;
        let result = super::part1(input)?;
        assert_eq!(result, 11);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let input = super::grammar::InputParser::new().parse(
            r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#,
        )?;
        let result = super::part2(input)?;
        assert_eq!(result, 31);
        Ok(())
    }
}
