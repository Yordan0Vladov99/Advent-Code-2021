use crate::snailfish::Snailfish::{Num, Pair};

#[derive(Clone)]
pub enum Snailfish {
    Num(u64),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {
    pub fn add(&self, other: &Snailfish) -> Snailfish {
        Snailfish::Pair(Box::new(self.clone()), Box::new(other.clone()))
    }

    pub fn to_text(&self) {
        match self {
            Num(val) => {
                print!("{}", *val);
            }
            Pair(left, right) => {
                print!("[");
                left.to_text();
                print!(",");
                right.to_text();
                print!("]")
            }
        }
    }
    pub fn from_text(text: &str) -> Snailfish {
        let mut i = 0usize;
        Snailfish::from_text_helper(text, &mut i)
    }

    fn from_text_helper(text: &str, index: &mut usize) -> Snailfish {
        *index += 1;
        let mut snailfish = text.chars().nth(*index).unwrap();
        let left = match snailfish {
            '0'..='9' => Num(snailfish.to_digit(10).unwrap().into()),
            _ => Snailfish::from_text_helper(text, index),
        };

        *index += 2;
        snailfish = text.chars().nth(*index).unwrap();
        let right = match snailfish {
            '0'..='9' => Num(snailfish.to_digit(10).unwrap().into()),
            _ => Snailfish::from_text_helper(text, index),
        };

        *index += 1;

        Pair(Box::new(left), Box::new(right))
    }

    pub fn reduce(&mut self) {
        loop {
            //self.to_text();
            //println!("\n");
            let mut has_exploded = false;
            self.explode(0, &mut has_exploded);
            if has_exploded {
                continue;
            }
            let has_split = self.split();
            if has_split {
                continue;
            }
            break;
        }
    }

    pub fn magnitude(&self) -> u64 {
        match self {
            Num(val) => *val,
            Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
    fn add_left_num(&mut self, num: u64) {
        match self {
            Num(val) => *self = Num(*val + num),
            Pair(left, _) => {
                left.add_left_num(num);
            }
        }
    }

    fn add_right_num(&mut self, num: u64) {
        match self {
            Num(val) => *self = Num(*val + num),
            Pair(_, right) => {
                right.add_right_num(num);
            }
        }
    }
    fn explode(&mut self, depth: u64, has_exploded: &mut bool) -> (u64, u64) {
        match self {
            Self::Num(_) => (0, 0),
            Self::Pair(left, right) => {
                if depth == 4 {
                    let (l, r) = match (*left.clone(), *right.clone()) {
                        (Num(val1), Num(val2)) => (val1, val2),
                        _ => panic!("Invalid state!"),
                    };
                    *has_exploded = true;
                    *self = Snailfish::Num(0u64);
                    return (l, r);
                }
                if !*has_exploded {
                    let l_res = left.explode(depth + 1, has_exploded);
                    if l_res != (0, 0) {
                        right.add_left_num(l_res.1);
                        return (l_res.0, 0);
                    }

                    let r_res = right.explode(depth + 1, has_exploded);
                    if r_res != (0, 0) {
                        left.add_right_num(r_res.0);
                        return (0, r_res.1);
                    }
                }
                (0, 0)
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Num(val) => {
                if *val >= 10 {
                    let left = *val / 2;
                    let right = match *val % 2 == 0 {
                        true => *val / 2,
                        false => *val / 2 + 1,
                    };
                    *self = Pair(Box::new(Num(left)), Box::new(Num(right)));
                    return true;
                }
                return false;
            }
            Pair(left, right) => {
                let l_res = left.split();
                if l_res {
                    return true;
                }

                return right.split();
            }
        }
    }
}
