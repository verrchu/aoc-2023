use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

#[derive(Debug, Clone, Copy)]
enum Param {
    X,
    M,
    A,
    S,
}

impl FromStr for Param {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Decision {
    Accept,
    Reject,
}

impl FromStr for Decision {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            s => Err(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Lt,
    Gt,
}

#[derive(Debug, Clone)]
enum Cond {
    Step {
        param: Param,
        op: Op,
        val: usize,
        next: String,
    },
    FinalStep {
        param: Param,
        op: Op,
        val: usize,
        next: Decision,
    },
    Next(String),
    Decision(Decision),
}

#[derive(Debug, Clone)]
struct Project {
    x: Vec<(usize, usize)>,
    m: Vec<(usize, usize)>,
    a: Vec<(usize, usize)>,
    s: Vec<(usize, usize)>,
}

impl Project {
    fn new() -> Self {
        Self {
            x: vec![(1, 4000)],
            m: vec![(1, 4000)],
            a: vec![(1, 4000)],
            s: vec![(1, 4000)],
        }
    }

    fn split(self, param: Param, op: Op, val: usize) -> (Self, Self) {
        use Op::*;
        use Param::*;

        let ranges = |old: &[(usize, usize)]| -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
            let mut yes = vec![];
            let mut no = vec![];

            for (a, b) in old.iter().copied() {
                match op {
                    Lt => {
                        if b < val {
                            yes.push((a, b));
                        } else if a < val {
                            yes.push((a, val - 1));
                            no.push((val, b));
                        } else {
                            no.push((a, b));
                        }
                    }
                    Gt => {
                        if a > val {
                            yes.push((a, b));
                        } else if b > val {
                            yes.push((val + 1, b));
                            no.push((a, val));
                        } else {
                            no.push((a, b));
                        }
                    }
                }
            }

            (yes, no)
        };

        match param {
            X => {
                let (yes, no) = ranges(&self.x);
                (
                    Self {
                        x: yes,
                        ..self.clone()
                    },
                    Self { x: no, ..self },
                )
            }
            M => {
                let (yes, no) = ranges(&self.m);
                (
                    Self {
                        m: yes,
                        ..self.clone()
                    },
                    Self { m: no, ..self },
                )
            }
            A => {
                let (yes, no) = ranges(&self.a);
                (
                    Self {
                        a: yes,
                        ..self.clone()
                    },
                    Self { a: no, ..self },
                )
            }
            S => {
                let (yes, no) = ranges(&self.s);
                (
                    Self {
                        s: yes,
                        ..self.clone()
                    },
                    Self { s: no, ..self },
                )
            }
        }
    }
}

fn solution(input: &str) -> usize {
    let mut lines = input.lines();

    let mut rules = HashMap::<String, Vec<Cond>>::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let (label, conds) = line.strip_suffix("}").unwrap().split_once("{").unwrap();

        let conds = conds
            .split(",")
            .map(|cond| match cond.split_once(":") {
                Some((cond, dst)) => {
                    let dst = dst.parse::<Decision>();

                    if let Some((p, v)) = cond.split_once('>') {
                        let p = p.parse::<Param>().unwrap();
                        let v = v.parse::<usize>().unwrap();

                        dst.map_or_else(
                            |l| Cond::Step {
                                param: p,
                                op: Op::Gt,
                                val: v,
                                next: l,
                            },
                            |d| Cond::FinalStep {
                                param: p,
                                op: Op::Gt,
                                val: v,
                                next: d,
                            },
                        )
                    } else if let Some((p, v)) = cond.split_once('<') {
                        let p = p.parse::<Param>().unwrap();
                        let v = v.parse::<usize>().unwrap();

                        dst.map_or_else(
                            |l| Cond::Step {
                                param: p,
                                op: Op::Lt,
                                val: v,
                                next: l,
                            },
                            |d| Cond::FinalStep {
                                param: p,
                                op: Op::Lt,
                                val: v,
                                next: d,
                            },
                        )
                    } else {
                        unreachable!()
                    }
                }
                None => cond
                    .parse::<Decision>()
                    .map_or_else(|l| Cond::Next(l), |d| Cond::Decision(d)),
            })
            .collect();

        rules.insert(label.to_string(), conds);
    }

    let mut ps = vec![];

    let mut progress = VecDeque::from_iter([("in", Project::new())]);
    while let Some((label, mut proj)) = progress.pop_front() {
        let conds = rules.get(label).unwrap();
        for cond in conds {
            match cond {
                Cond::Step {
                    param,
                    op,
                    val,
                    next,
                } => {
                    let (yes, no) = proj.clone().split(*param, *op, *val);
                    progress.push_back((next, yes));
                    proj = no;
                }
                Cond::FinalStep {
                    param,
                    op,
                    val,
                    next,
                } => {
                    let (yes, no) = proj.split(*param, *op, *val);
                    if *next == Decision::Accept {
                        ps.push(yes);
                    }

                    proj = no;
                }
                Cond::Next(next) => {
                    progress.push_back((next, proj.clone()));
                }
                Cond::Decision(d) => {
                    if *d == Decision::Accept {
                        ps.push(proj.clone());
                    }
                }
            }
        }
    }

    ps.into_iter()
        .map(|p| {
            let f = |rs: Vec<(usize, usize)>| rs.into_iter().map(|(a, b)| b - a + 1).sum::<usize>();
            f(p.x) * f(p.m) * f(p.a) * f(p.s)
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 167409079868000);
    }
}
