use anyhow::{anyhow, Result};
use nom::IResult;

const DECK_SIZE: usize = 10007;

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
    let techniques = parse_input(input)
        .map_err(|e| anyhow!("Failed to parse input: {e}"))?
        .1;
    // The LinearShuffle was created for part 2, see below
    let shuffle: LinearShuffle<DECK_SIZE> = LinearShuffle::from(techniques.iter().copied());
    let card = shuffle.placement(2019);
    Ok(format!("{card}"))
}

// PART 2 SKETCHPAD
// Completely different approach needed here;
// deck size: 119315717514047
// repeated shuffles: 101741582076661
// Let's try to express the shuffles differently, since we only need to know
// a single card to win (the one in position 2020)
// Ideally we would get a function f(x) = ax + b where x is the position of the card
// and f(x) is the position of the card after the shuffle for each possible deck transformation
// and finding a way to compose them together. We'll worry later about how to compose them together
// a bajillion times, let's try to figure out how to do 1 shuffle first.
// The reverse should be easy; it is f(x) = N - 1 - x where N is the deck size. It is its own inverse.
// We can easily see that by plugging it in for x in the original function:
// f(f(x)) = N - 1 - (N - 1 - x) = x
// For reverse, a = -1, b = N - 1
// Let's try to figure out cut next. We can see that cut(n) is f(x) = (x - n) (but mod N), in other words
// for cut n, a = 1, b = -n. The inverse would be f(x) = (x + n), so a = 1, b = n, let's
// make sure by substituting: f(f(x)) = ((x + n) - n) = x.
// Next up, deal with increment n would be:
// f(x) = n * x, but mod N. In other words, a = n, b = 0. Don't rightly know how to invert this yet.
// Anyhow, suppose we have f(x) and g(x) and we want to compose them, we'll insert g(x) for x in f(x).
// Let's say the coefficients of f are a_1 and b_1 and the coefficients of g are a_2 and b_2, then:
// f(x) = a_1 * x + b_1, g(x) = a_2 * x + b_2
// h(x) = f(g(x)) = a_1 * (a_2 * x + b_2) + b_1 = a_1 * a_2 * x + a_1 * b_2 + b_1
// so a = a_1 * a_2, b = a_1 * b_2 + b_1
// We also have that new deck order is f(x) = x, so a = 1, b = 0
// Our final h(x) is a function that takes a card position and returns the final position of the card
// so we can't directly compare the result of calling h(x) with a shuffled deck from earlier, which
// results in a deck with all the cards placed in the correct positions.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct LinearShuffle<const N: usize> {
    // Would think i64 is enough, but it actually overflows somewhere and creates errors
    scale: i128,
    shift: i128,
}

impl<const N: usize> LinearShuffle<N> {
    fn default() -> Self {
        LinearShuffle { scale: 1, shift: 0 }
    }

    fn cut_n(n: i16) -> Self {
        LinearShuffle {
            scale: 1,
            shift: -(n as i128),
        }
    }
    fn deal_into_new_stack() -> Self {
        LinearShuffle {
            scale: -1,
            shift: (N as i128) - 1,
        }
    }
    fn deal_with_increment_n(n: u16) -> Self {
        LinearShuffle {
            scale: n as i128,
            shift: 0,
        }
    }
    fn compose(self, other: Self) -> Self {
        LinearShuffle {
            scale: (self.scale * other.scale) % (N as i128),
            shift: (other.scale * self.shift + other.shift) % (N as i128),
        }
        .normalize()
    }
    fn placement(self, x: i128) -> i128 {
        (x * self.scale + self.shift).rem_euclid(N as i128)
    }

    fn from(techniques: impl Iterator<Item = Technique>) -> Self {
        techniques.fold(Self::default(), |acc, technique| {
            acc.compose(technique.into())
        })
    }

    /// Basically exponentiation by repeated squaring
    fn repeat(self, times: u64) -> LinearShuffle<N> {
        if times == 0 {
            LinearShuffle::default()
        } else if times == 1 {
            self
        } else if times.is_multiple_of(2) {
            self.compose(self).repeat(times / 2)
        } else {
            self.compose(self.compose(self).repeat(times / 2))
        }
    }

    fn normalize(self) -> LinearShuffle<N> {
        LinearShuffle {
            scale: self.scale.rem_euclid(N as i128),
            shift: self.shift.rem_euclid(N as i128),
        }
    }

    fn invert(self) -> LinearShuffle<N> {
        let target = self.normalize();
        let (gcd, a, _) = extended_gcd(target.scale, N as i128);
        assert!(gcd == 1 || gcd == -1);
        LinearShuffle {
            scale: a,
            shift: -a * target.shift,
        }
        .normalize()
    }
}

impl<const N: usize> From<Technique> for LinearShuffle<N> {
    fn from(technique: Technique) -> Self {
        match technique {
            Technique::DealIntoNewStack => Self::deal_into_new_stack(),
            Technique::Cut(n) => Self::cut_n(n),
            Technique::DealWithIncrement(n) => Self::deal_with_increment_n(n),
        }
    }
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        if b < 0 {
            (-b, 0, -1)
        } else {
            (b, 0, 1)
        }
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn part_2(input: &str) -> Result<String> {
    let techniques = parse_input(input)
        .map_err(|e| anyhow!("Failed to parse input: {e}"))?
        .1;
    let shuffle: LinearShuffle<119315717514047> = LinearShuffle::from(techniques.into_iter());
    let shuffle_repeats: u64 = 101741582076661;
    let final_shuffle = shuffle.repeat(shuffle_repeats).normalize();
    let invert = final_shuffle.invert();
    assert_eq!(final_shuffle.compose(invert), LinearShuffle::default());
    Ok(format!("{}", invert.placement(2020)))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use quickcheck::{quickcheck, Arbitrary, Gen};
    use std::collections::VecDeque;
    // Require N to be a prime number for LinearShuffle<N>::invert to work in the general case
    // (From modular multiplicative inverse)
    const TEST_DECK_SIZE: usize = 1597;

    impl Arbitrary for Technique {
        fn arbitrary(g: &mut Gen) -> Self {
            let choice = u8::arbitrary(g).rem_euclid(3);
            match choice {
                0 => Technique::DealIntoNewStack,
                1 => Technique::Cut(i16::arbitrary(g) % (TEST_DECK_SIZE as i16)),
                2 => loop {
                    let n = u16::arbitrary(g) % (TEST_DECK_SIZE as u16);
                    if n != 0 {
                        return Technique::DealWithIncrement(n);
                    }
                },
                _ => unreachable!(),
            }
        }
    }

    quickcheck! {
        fn compose_inverted_shuffle_with_itself_yields_identity(shuffle: Vec<Technique> ) -> bool {
            let shuf: LinearShuffle<TEST_DECK_SIZE> = LinearShuffle::from(shuffle.into_iter());
            let inv = shuf.invert();
            shuf.compose(inv) == LinearShuffle::default()
        }

        fn any_shuffle_move_is_equivalent_to_its_technique_shuffle(technique: Technique) -> bool {
            let shuf: LinearShuffle<TEST_DECK_SIZE> = technique.into();
            let mut deck = Deck::new(TEST_DECK_SIZE as u16);
            deck.apply_shuffles(&[technique]);
            deck.cards.into_iter().enumerate().all(|(pos, card)|
                shuf.placement(card as i128) == pos as i128
            )
        }

        fn any_combination_of_shuffle_moves_is_equal_to_its_technique_shuffle(shuffle: Vec<Technique>) -> bool {
            let mut deck = Deck::new(TEST_DECK_SIZE as u16);
            deck.apply_shuffles(&shuffle);
            let shuf: LinearShuffle<TEST_DECK_SIZE> = LinearShuffle::from(shuffle.into_iter());
            deck.cards.into_iter().enumerate().all(|(pos, card)|
                shuf.placement(card as i128) == pos as i128
            )
        }
    }

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
                // Cut n cards from bottom of deck and place at top
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
}
