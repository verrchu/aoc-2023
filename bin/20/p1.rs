use std::collections::{HashMap, VecDeque};

static INPUT: &str = include_str!("./input");

fn main() {
    println!("result: {}", solution(INPUT));
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Mod {
    F {
        enabled: bool,
        dst: Vec<String>,
    },
    C {
        src: Vec<(Pulse, String)>,
        dst: Vec<String>,
    },
}

impl Mod {
    fn f() -> Self {
        Self::F {
            enabled: false,
            dst: vec![],
        }
    }

    fn c() -> Self {
        Self::C {
            src: vec![],
            dst: vec![],
        }
    }

    fn with_dst(self, dst: Vec<String>) -> Self {
        match self {
            Self::F { enabled, .. } => Self::F { enabled, dst },
            Self::C { src, .. } => Self::C { src, dst },
        }
    }

    fn dst(&self) -> &[String] {
        match self {
            Self::F { dst, .. } => dst,
            Self::C { dst, .. } => dst,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    L,
    H,
}

fn solution(input: &str) -> usize {
    let mut broadcast = vec![];
    let mut state = HashMap::new();

    for line in input.lines() {
        let (src, dst) = line.split_once(" -> ").unwrap();
        let dst = dst.split(", ").map(|d| d.to_string()).collect::<Vec<_>>();

        if let Some(src) = src.strip_prefix('%') {
            state.insert(src, Mod::f().with_dst(dst));
        } else if let Some(src) = src.strip_prefix('&') {
            state.insert(src, Mod::c().with_dst(dst));
        } else {
            broadcast = dst;
        }
    }

    let dst = state
        .iter()
        .map(|(m, s)| (m.to_string(), s.dst().to_vec()))
        .collect::<Vec<_>>();
    for (m, dst) in dst {
        for d in dst {
            if let Some(Mod::C { src, .. }) = state.get_mut(d.as_str()) {
                src.push((Pulse::L, m.to_string()));
            }
        }
    }

    let mut l = 0;
    let mut h = 0;
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.extend(
            broadcast
                .iter()
                .map(|dst| (Pulse::L, "broadcast".to_string(), dst.to_string())),
        );

        l += 1;
        while let Some((p, prev, curr)) = pulses.pop_front() {
            use Mod::*;
            use Pulse::*;

            match p {
                L => l += 1,
                H => h += 1,
            }

            let Some(m) = state.get_mut(curr.as_str()) else {
                continue;
            };

            match (p, m) {
                (L, F { enabled, dst }) => {
                    *enabled = !*enabled;
                    let p = enabled.then_some(H).unwrap_or(L);
                    pulses.extend(dst.iter().map(|dst| (p, curr.to_string(), dst.to_string())));
                }
                (p, C { src, dst }) => {
                    let (pp, _) = src
                        .iter_mut()
                        .find(|(_, src)| src.as_str() == prev.as_str())
                        .unwrap();
                    *pp = p;

                    let p = src.iter().all(|(p, _)| *p == H).then_some(L).unwrap_or(H);
                    pulses.extend(dst.iter().map(|dst| (p, curr.to_string(), dst.to_string())));
                }
                _ => continue,
            }
        }
    }

    l * h
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case(include_str!("./example-1"), 32000000; "1")]
    #[test_case(include_str!("./example-2"), 11687500; "2")]
    fn example(input: &str, val: usize) {
        assert_eq!(solution(input), val);
    }
}
