use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt");
    println!("p1: {}", p1(input)?);
    println!("p2: {}", p2(input)?);
    Ok(())
}

pub fn p1(s: &str) -> anyhow::Result<u32> {
    let input: Program = s.parse()?;
    Ok(input.eval_p1())
}

pub fn p2(s: &str) -> anyhow::Result<u32> {
    let input: Program = s.parse()?;
    Ok(input.eval_p2())
}

struct Program {
    instrs: Vec<Instruction>,
}

impl Program {
    pub fn eval_p1(&self) -> u32 {
        self.instrs
            .iter()
            .filter_map(|i| match i {
                Instruction::Mul(a, b) => Some(a * b),
                _ => None,
            })
            .sum()
    }

    pub fn eval_p2(&self) -> u32 {
        let mut result = 0;
        let mut enabled = true;

        for instr in &self.instrs {
            match instr {
                Instruction::Do => {
                    enabled = true;
                }
                Instruction::Dont => {
                    enabled = false;
                }
                Instruction::Mul(a, b) => {
                    result += a * b * u32::from(enabled);
                }
            }
        }

        result
    }
}

impl FromStr for Program {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mul = r#"mul\(([0-9]+),([0-9]+)\)"#;
        let do_ = r#"do\(\)"#;
        let dont = r#"don't\(\)"#;
        let full = format!("({mul})|({do_})|({dont})");
        let re = regex::Regex::new(&full).unwrap();

        let instrs: Vec<Instruction> = re
            .captures_iter(s)
            .map(|cap| -> anyhow::Result<Instruction> {
                if let Some(_) = cap.get(1) {
                    let left: u32 = cap.get(2).unwrap().as_str().parse()?;
                    let right: u32 = cap.get(3).unwrap().as_str().parse()?;
                    Ok(Instruction::Mul(left, right))
                } else if let Some(_) = cap.get(4) {
                    Ok(Instruction::Do)
                } else {
                    cap.get(5).unwrap();
                    Ok(Instruction::Dont)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { instrs })
    }
}

#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_p1() -> anyhow::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let res = super::p1(input)?;
        assert_eq!(res, 161);

        Ok(())
    }

    #[test]
    fn test_p2() -> anyhow::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let res = super::p2(input)?;
        assert_eq!(res, 48);

        Ok(())
    }
}
