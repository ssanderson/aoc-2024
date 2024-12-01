#[derive(Debug, Clone)]
pub struct Input(pub Vec<Line>);

impl Input {
    pub fn split(self) -> (Vec<u32>, Vec<u32>) {
        let mut l = Vec::new();
        let mut r = Vec::new();
        for Line(a, b) in self.0 {
            l.push(a);
            r.push(b);
        }
        (l, r)
    }
}

#[derive(Debug, Clone)]
pub struct Line(pub i32, pub i32);
