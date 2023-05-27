use anyhow::{anyhow, Context, Result};
use nom::IResult;
use std::collections::VecDeque;

const DECK_SIZE: u16 = 10007;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Deck {
    cards: VecDeque<u16>,
    shuffle_area: Vec<u16>,
}

impl Deck {
    fn new(cards: u16) -> Self {
        Deck {
            cards: (0..cards).collect(),
            shuffle_area: vec![0; cards as usize],
        }
    }

    fn deal_into_new_stack(self: &mut Deck) {
        // Deal entire deck from top into new stack; effectively reversing it
        self.cards.make_contiguous().reverse();
    }

    fn cut(self: &mut Deck, n: i16) {
        // Retain order of the cut in both cases; then:
        if n > 0 {
            // Cut n cards from top of deck and place at bottom
            self.cards.rotate_left(n as usize);
        } else {
            // Cut n cards from bottom of deck and place at bottom
            self.cards.rotate_right(-n as usize);
        }
    }

    fn deal_with_increment(self: &mut Deck, n: u16) {
        let mut pos = 0; // start at left of shuffle area
        while let Some(top) = self.cards.pop_front() {
            assert_eq!(self.shuffle_area[pos], 0);
            self.shuffle_area[pos] = top;
            pos = (pos + n as usize) % self.shuffle_area.len();
        }
        self.cards.extend(self.shuffle_area.iter());
        self.shuffle_area.iter_mut().for_each(|c| *c = 0);
    }

    fn shuffle(self: &mut Deck, technique: Technique) {
        match technique {
            Technique::DealIntoNewStack => self.deal_into_new_stack(),
            Technique::Cut(n) => self.cut(n),
            Technique::DealWithIncrement(n) => self.deal_with_increment(n),
        }
    }

    fn apply_shuffles(self: &mut Deck, shuffles: &[Technique]) {
        for shuffle in shuffles {
            self.shuffle(*shuffle);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Technique {
    DealIntoNewStack,
    Cut(i16),
    DealWithIncrement(u16),
}

fn parse_deal_into_new_stack(input: &str) -> IResult<&str, Technique> {
    let (input, _) = nom::bytes::complete::tag("deal into new stack")(input)?;
    Ok((input, Technique::DealIntoNewStack))
}

fn parse_cut(input: &str) -> IResult<&str, Technique> {
    let (input, _) = nom::bytes::complete::tag("cut ")(input)?;
    let (input, n) = nom::character::complete::i16(input)?;
    Ok((input, Technique::Cut(n)))
}

fn parse_deal_with_increment(input: &str) -> IResult<&str, Technique> {
    let (input, _) = nom::bytes::complete::tag("deal with increment ")(input)?;
    let (input, n) = nom::character::complete::u16(input)?;
    Ok((input, Technique::DealWithIncrement(n)))
}

fn parse_technique(input: &str) -> IResult<&str, Technique> {
    nom::branch::alt((
        parse_deal_into_new_stack,
        parse_cut,
        parse_deal_with_increment,
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Technique>> {
    nom::multi::separated_list1(nom::character::complete::line_ending, parse_technique)(input)
}

pub fn part_1(input: &str) -> Result<String> {
    let mut deck = Deck::new(DECK_SIZE);
    let techniques = parse_input(input)
        .map_err(|e| anyhow!("Failed to parse input: {e}"))?
        .1;
    deck.apply_shuffles(&techniques);
    deck.cards
        .iter()
        .position(|card| *card == 2019)
        .with_context(|| "Card not found")
        .map(|n| format!("{n}"))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn run_example_test(input: &str, expected: Vec<u16>) {
        let mut deck = Deck::new(10);
        let techniques = parse_input(input).unwrap().1;
        deck.apply_shuffles(&techniques);
        assert_eq!(deck.cards, expected);
    }

    #[test]
    fn first_example() {
        let input = "deal with increment 7
deal into new stack
deal into new stack";
        run_example_test(input, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7])
    }

    #[test]
    fn test_second_example() {
        let input = "cut 6
deal with increment 7
deal into new stack";
        run_example_test(input, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_third_example() {
        let input = "deal with increment 7
deal with increment 9
cut -2";
        run_example_test(input, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_fourth_example() {
        let input = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        run_example_test(input, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn test_deal_into_new_stack() {
        let mut deck = Deck::new(10);
        deck.deal_into_new_stack();
        assert_eq!(deck.cards, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_deal_with_increment() {
        let mut deck = Deck::new(10);
        deck.deal_with_increment(3);
        assert_eq!(deck.cards, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_cut_top() {
        let mut deck = Deck::new(10);
        deck.cut(3);
        assert_eq!(deck.cards, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_cut_bot() {
        let mut deck = Deck::new(10);
        deck.cut(-4);
        assert_eq!(deck.cards, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }
}
