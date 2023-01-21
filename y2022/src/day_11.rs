use anyhow::Result;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square(),
}

#[derive(PartialEq, Debug)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    op: Operation,
    divides_by: u64,
    pos_monkey: usize,
    neg_monkey: usize,
}

const MONKEY_RE: &str = r"Monkey ([0-9]+):\n  Starting items: (.+)\n  Operation: new = (.+)\n  Test: divisible by ([0-9]+)\n    If true: throw to monkey ([0-9]+)\n    If false: throw to monkey ([0-9]+)";

fn parse_monkey_block(monkey: &str) -> Option<Monkey> {
    let re = regex::Regex::new(MONKEY_RE).ok()?;
    let groups = re.captures(monkey)?;
    let id: usize = groups.get(1)?.as_str().parse().ok()?;

    let starting_items_text: &str = groups.get(2)?.as_str();
    let items: Vec<u64> = starting_items_text
        .split(", ")
        .filter_map(|it| it.parse().ok())
        .collect();

    let op_text = groups.get(3)?.as_str();
    let op = if op_text == "old * old" {
        Some(Operation::Square())
    } else {
        let mut parts = op_text.split(' ');
        parts.next();
        let operator = parts.next()?;
        let operand = parts.next()?.parse().ok()?;
        if operator == "*" {
            Some(Operation::Mul(operand))
        } else if operator == "+" {
            Some(Operation::Add(operand))
        } else {
            None
        }
    }?;

    let divides_by: u64 = groups.get(4)?.as_str().parse().ok()?;
    let pos_monkey: usize = groups.get(5)?.as_str().parse().ok()?;
    let neg_monkey: usize = groups.get(6)?.as_str().parse().ok()?;

    Some(Monkey {
        id,
        items,
        op,
        divides_by,
        pos_monkey,
        neg_monkey,
    })
}

fn parse_monkeys(text: &str) -> Vec<Monkey> {
    text.split("\n\n").filter_map(parse_monkey_block).collect()
}

fn new_value(old: u64, op: &Operation) -> u64 {
    match op {
        Operation::Square() => old * old,
        Operation::Mul(arg) => old * arg,
        Operation::Add(arg) => old + arg,
    }
}

fn do_monkey_turn(
    monkey_id: usize,
    monkeys: &mut [Monkey],
    counter: &mut HashMap<usize, usize>,
    modulo: bool,
) {
    let base = monkeys.iter().fold(1, |product, m| product * m.divides_by);

    let monkey = &mut monkeys[monkey_id];
    let mut moves: Vec<(usize, u64)> = Vec::new();

    for item in &monkey.items {
        let new_worry = new_value(*item, &monkey.op);
        let worry = if modulo {
            new_worry % base
        } else {
            new_worry / 3
        };
        let target_monkey = if worry % monkey.divides_by == 0 {
            monkey.pos_monkey
        } else {
            monkey.neg_monkey
        };
        *counter.entry(monkey_id).or_insert(0) += 1;
        moves.push((target_monkey, worry));
    }
    monkey.items.clear();

    for (target_monkey, item) in moves {
        monkeys[target_monkey].items.push(item);
    }
}

fn do_monkey_round(monkeys: &mut Vec<Monkey>, counter: &mut HashMap<usize, usize>, modulo: bool) {
    for monkey_id in 0..monkeys.len() {
        do_monkey_turn(monkey_id, monkeys, counter, modulo);
    }
}

fn monkey_game(input: &str, rounds: usize, modulo: bool) -> usize {
    let mut monkeys = parse_monkeys(input);
    let mut counter = HashMap::new();
    for _ in 0..rounds {
        do_monkey_round(&mut monkeys, &mut counter, modulo);
    }
    let mut counts: Vec<usize> = counter.values().cloned().collect();
    counts.sort();
    counts[counts.len() - 1] * counts[counts.len() - 2]
}

pub fn part_1(input: &str) -> Result<String> {
    let sol = monkey_game(input, 20, false);
    Ok(format!("{sol}"))
}

pub fn part_2(input: &str) -> Result<String> {
    let sol = monkey_game(input, 10000, true);
    Ok(format!("{sol}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_parse_monkey_block() {
        let input = EXAMPLE.split("\n\n").next().unwrap();
        let maybe_monkey = parse_monkey_block(input);
        assert!(maybe_monkey.is_some());
        let monkey = maybe_monkey.unwrap();
        assert_eq!(monkey.id, 0);
        assert_eq!(monkey.items, vec![79, 98]);
        assert_eq!(monkey.op, Operation::Mul(19));
        assert_eq!(monkey.divides_by, 23);
        assert_eq!(monkey.pos_monkey, 2);
        assert_eq!(monkey.neg_monkey, 3);
    }

    #[test]
    fn test_parse_monkeys() {
        let monkeys = parse_monkeys(EXAMPLE);
        assert_eq!(monkeys.len(), 4);
    }

    #[test]
    fn test_monkey_turn() {
        let mut monkeys = parse_monkeys(EXAMPLE);
        do_monkey_turn(0, &mut monkeys, &mut HashMap::new(), false);
        println!("{:?}", monkeys[0])
    }

    #[test]
    fn test_monkey_game() {
        let monkey_business = monkey_game(EXAMPLE, 20, false);
        assert_eq!(monkey_business, 10605);
    }
}
