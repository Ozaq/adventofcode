use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Op {
    Add(i64),
    Mul(i64),
    Square,
}

impl Op {
    fn apply(&self, val: i64) -> i64 {
        match self {
            Op::Add(x) => val + x,
            Op::Mul(x) => val * x,
            Op::Square => val * val,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old * old" => Ok(Op::Square),
            s if s.starts_with("old *") => Ok(Op::Mul(
                s.split_once('*').unwrap().1.trim().parse::<i64>().unwrap(),
            )),
            s if s.starts_with("old +") => Ok(Op::Add(
                s.split_once('+').unwrap().1.trim().parse::<i64>().unwrap(),
            )),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    div_by: i64,
    first_target: usize,
    second_target: usize,
    inspection_count: usize,
}

impl Monkey {
    fn new(lines: &[String]) -> Self {
        let items = lines[1]
            .trim()
            .strip_prefix("Starting items:")
            .unwrap()
            .split(',')
            .map(|x| x.trim())
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let op = Op::from_str(lines[2].split_once('=').unwrap().1.trim()).unwrap();
        let div_by = lines[3]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let first_target = lines[4]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let second_target = lines[5]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        Self {
            items,
            op,
            div_by,
            first_target,
            second_target,
            inspection_count: 0,
        }
    }
    fn do_turn<F>(&mut self, op: F) -> (Vec<i64>, Vec<i64>)
    where
        F: Fn(i64) -> i64,
    {
        let (mut a, mut b) = (vec![], vec![]);
        for item in &self.items {
            self.inspection_count += 1;
            let worry_level = op(self.op.apply(*item));
            if worry_level % self.div_by == 0 {
                a.push(worry_level);
            } else {
                b.push(worry_level);
            }
        }
        self.items.clear();
        (a, b)
    }
}

#[derive(Debug, Clone)]
struct Circus {
    monkeys: Vec<Monkey>,
}

impl Circus {
    fn new(lines: &Vec<String>) -> Self {
        let mut monkeys = Vec::new();
        for idx in 0..lines.len() / 6 {
            monkeys.push(Monkey::new(&lines[idx * 6..(idx + 1) * 6]));
        }
        Self { monkeys }
    }
    fn do_round<F>(&mut self, op: F)
    where
        F: Fn(i64) -> i64 + Copy,
    {
        for idx in 0..self.monkeys.len() {
            let monkey = self.monkeys.get_mut(idx).unwrap();
            let (mut a, mut b) = monkey.do_turn(op);
            let a_idx = monkey.first_target;
            let b_idx = monkey.second_target;
            self.monkeys.get_mut(a_idx).unwrap().items.append(&mut a);
            self.monkeys.get_mut(b_idx).unwrap().items.append(&mut b);
        }
    }
}

fn pt1(mut circus: Circus) {
    for _ in 0..20 {
        circus.do_round(|x| x / 3);
    }
    let mut important_monkeys = circus
        .monkeys
        .iter()
        .map(|m| m.inspection_count)
        .collect::<Vec<_>>();
    important_monkeys.sort_by(|a, b| b.cmp(a));
    let mut monkey_business = 1;
    for activity in important_monkeys.iter().take(2) {
        monkey_business *= activity;
    }
    println!("Level of monkey business (relaxed) = {monkey_business}");
}

fn pt2(mut circus: Circus) {
    let module = circus
        .monkeys
        .iter()
        .map(|x| x.div_by)
        .reduce(|acc, x| acc * x)
        .unwrap();
    for _ in 0..10000 {
        circus.do_round(|x| x % module);
    }
    let mut important_monkeys = circus
        .monkeys
        .iter()
        .map(|m| m.inspection_count)
        .collect::<Vec<_>>();
    important_monkeys.sort_by(|a, b| b.cmp(a));
    let mut monkey_business = 1;
    for activity in important_monkeys.iter().take(2) {
        println!("A: {activity}");
        monkey_business *= activity;
    }
    println!("Level of monkey business (stresssed) = {monkey_business}");
}

fn main() {
    let filename = env::args().nth(1).expect("Missing filename argument");
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file)
        .lines()
        .flatten()
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();
    let circus = Circus::new(&lines);
    pt1(circus.clone());
    pt2(circus);
}
