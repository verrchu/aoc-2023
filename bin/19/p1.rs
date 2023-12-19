use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
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

fn solution(input: &str) -> usize {
    let mut lines = input.lines();

    let mut rules = HashMap::<String, VecDeque<Cond>>::new();
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

    let parts = lines.map(|line| {
        let mut line = line
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(",");

        let mut next = || {
            let (_, n) = line.next().unwrap().split_once("=").unwrap();
            n.parse::<usize>().unwrap()
        };

        Part {
            x: next(),
            m: next(),
            a: next(),
            s: next(),
        }
    });

    let mut result = 0;

    fn eval(part: &Part, param: Param, op: Op, val: usize) -> bool {
        use Op::*;
        use Param::*;

        match (param, op) {
            (X, Gt) => part.x > val,
            (M, Gt) => part.m > val,
            (A, Gt) => part.a > val,
            (S, Gt) => part.s > val,
            (X, Lt) => part.x < val,
            (M, Lt) => part.m < val,
            (A, Lt) => part.a < val,
            (S, Lt) => part.s < val,
        }
    }

    for part in parts {
        let mut conds = rules.get("in").unwrap().clone();
        while let Some(cond) = conds.pop_front() {
            match cond {
                Cond::Step {
                    param,
                    op,
                    val,
                    next,
                } => {
                    if eval(&part, param, op, val) {
                        conds.drain(..);
                        conds.extend(rules.get(&next).unwrap().into_iter().cloned());
                    }
                }
                Cond::FinalStep {
                    param,
                    op,
                    val,
                    next,
                } => {
                    if eval(&part, param, op, val) {
                        if next == Decision::Accept {
                            result += part.x + part.m + part.a + part.s;
                        }

                        break;
                    }
                }
                Cond::Next(next) => {
                    conds.drain(..);
                    conds.extend(rules.get(&next).unwrap().into_iter().cloned());
                }
                Cond::Decision(d) => {
                    if d == Decision::Accept {
                        result += part.x + part.m + part.a + part.s;
                    }

                    break;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("./example");

    #[test]
    fn example() {
        assert_eq!(solution(EXAMPLE), 19114);
    }
}
