use std::collections::HashMap;
use std::fmt;

use rand::prelude::*;

type Deck = Vec<Card>;

#[derive(Clone, Copy)]
enum Suits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suits::Clubs => write!(f, "clubs"),
            Suits::Diamonds => write!(f, "diamonds"),
            Suits::Hearts => write!(f, "hearts"),
            Suits::Spades => write!(f, "spades"),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Values {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Values::Ace => write!(f, "Ace"),
            Values::Two => write!(f, "Two"),
            Values::Three => write!(f, "Three"),
            Values::Four => write!(f, "Four"),
            Values::Five => write!(f, "Five"),
            Values::Six => write!(f, "Six"),
            Values::Seven => write!(f, "Seven"),
            Values::Eight => write!(f, "Eight"),
            Values::Nine => write!(f, "Nine"),
            Values::Ten => write!(f, "Ten"),
            Values::Jack => write!(f, "Jack"),
            Values::Queen => write!(f, "Queen"),
            Values::King => write!(f, "King"),
        }
    }
}

struct Card {
    suit: Suits,
    value: Values,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}.", self.value, self.suit)
    }
}

fn make_deck() -> Deck {
    let suit_map = HashMap::from([
        (0, Suits::Clubs),
        (1, Suits::Diamonds),
        (2, Suits::Hearts),
        (3, Suits::Spades),
    ]);
    let value_map = HashMap::from([
        (0, Values::Ace),
        (1, Values::Two),
        (2, Values::Three),
        (3, Values::Four),
        (4, Values::Five),
        (5, Values::Six),
        (6, Values::Seven),
        (7, Values::Eight),
        (8, Values::Nine),
        (9, Values::Ten),
        (10, Values::Jack),
        (11, Values::Queen),
        (12, Values::King),
    ]);

    let mut deck: Deck = Vec::with_capacity(52);

    for v in 0..13 {
        let value = value_map.get(&v).unwrap().to_owned();

        for s in 0..4 {
            let suit = suit_map.get(&s).unwrap().to_owned();
            deck.push(Card { suit, value });
        }
    }
    deck
}

fn print_card(card: &Card) {
    println!("{card}");
}

fn print_deck(deck: &Deck) {
    for card in deck {
        print_card(card);
    }
}

fn shuffle(deck: &mut Deck) {
    let mut rng = thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(0_usize, 51_usize);

    for _ in 0..50 {
        for i in 0..deck.len() {
            let new_i = rng.sample(distr);
            deck.swap(i, new_i);
        }
    }
}

fn main() {
    let mut deck = make_deck();
    print_deck(&deck);
    println!("\nWe be shufflin...shufflin...\n");
    shuffle(&mut deck);
    print_deck(&deck);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_size() {
        let deck = make_deck();
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn test_deck_order() {
        let deck = make_deck();
        for i in (0..52).step_by(4) {
            assert!(
                (deck[i].value == deck[i + 1].value) == (deck[i + 2].value == deck[i + 3].value),
                "Comparing {} through {}",
                deck[i],
                deck[i + 3]
            );
        }
    }
}
