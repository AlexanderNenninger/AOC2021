use std::default::Default;
use std::ops::Add;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct State {
    pos: Vec2,
    vel: Vec2,
}

#[derive(Debug, Clone, Copy)]
struct Target {
    ll: Vec2,
    ur: Vec2,
}

impl Target {
    fn check(&self, s: &State) -> bool {
        self.ll <= s.pos && s.pos <= self.ur
    }

    fn points(&self) -> Vec<Vec2> {
        let mut out = vec![];
        let mut p = self.ll;
        while p.x <= self.ur.x {
            while p.y <= self.ur.y {
                out.push(p);
                p.y += 1;
            }
            p.y = self.ll.y;
            p.x += 1
        }
        out
    }
}

#[derive(Debug)]
struct Program {
    init: State,
    target: Target,
}

impl Program {
    fn solve_forward(&self, n: isize) -> State {
        State {
            pos: Vec2 {
                x: self.init.pos.x + (0..n).map(|i| 0.max(self.init.vel.x - i)).sum::<isize>(),
                y: self.init.pos.x + self.init.vel.x - n * (n + 1) / 2,
            },
            vel: Vec2 {
                x: 0.max(self.init.vel.x - n),
                y: self.init.vel.y - n,
            },
        }
    }

    fn solve_backward(&mut self) -> Vec<State> {
        let mut solved = false;
        let mut n = self.target.ll.x; // Highest point is when we have the minimal initial x velocity
        let mut solutions: Vec<State> = vec![];
        while !solved {
            for t in self.target.points().iter() {
                let x = t.x - n * (n + 1) / 2;
                let y = t.y - n * (n + 1) / 2;

                self.init = State {
                    pos: Vec2::default(),
                    vel: Vec2 { x, y },
                };
                solved = self.check(n);

                if solved {
                    solutions.push(self.init)
                }
            }
            n -= 1;
        }
        solutions
    }

    fn check(&self, n: isize) -> bool {
        let t = self.solve_forward(n);
        self.target.check(&t)
    }
}

mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_forward_7() {
        let init = State {
            pos: Vec2::default(),
            vel: Vec2 { x: 7, y: 2 },
        };

        let target = Target {
            ll: Vec2 { x: 20, y: -10 },
            ur: Vec2 { x: 30, y: -5 },
        };

        let prog = Program { init, target };
        assert!(prog.check(7));
    }

    #[test]
    fn test_forward_10() {
        let init = State {
            pos: Vec2::default(),
            vel: Vec2 { x: 6, y: 3 },
        };

        let target = Target {
            ll: Vec2 { x: 20, y: -10 },
            ur: Vec2 { x: 30, y: -5 },
        };

        let prog = Program { init, target };
        assert!(prog.check(10));
    }

    #[test]
    fn test_target() {
        let t = Target {
            ll: Vec2 { x: 0, y: 0 },
            ur: Vec2 { x: 1, y: 1 },
        };

        let expected = vec![
            Vec2 { x: 0, y: 0 },
            Vec2 { x: 0, y: 1 },
            Vec2 { x: 1, y: 0 },
            Vec2 { x: 1, y: 1 },
        ];

        assert_eq!(t.points(), expected);
    }

    #[test]
    fn test_solve_backward() {
        let t = Target {
            ll: Vec2 { x: 20, y: -10 },
            ur: Vec2 { x: 30, y: -5 },
        };

        let mut prog = Program {
            target: t,
            init: State::default(),
        };

        let solutions = prog.solve_backward();

        for s in solutions.iter() {
            println!("{:?}", s);
        }
    }
}
