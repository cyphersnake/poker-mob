use std::cmp::Ordering;
use std::convert::TryFrom;

// TODO Make it enum
type Value = u8;

#[derive(Debug, PartialEq)]
enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}

#[derive(Debug, PartialEq)]
struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    fn new(suit: Suit, value: Value) -> Self {
        Self { suit, value }
    }
}

#[derive(Debug)]
struct Hand([Card; 5]);

#[derive(PartialEq, Debug)]
enum Combination {
    HighCard(Card, Hand),
    Flash(Suit, Card),
}

impl TryFrom<Hand> for Combination {
    type Error = ();
    fn try_from(hand: Hand) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl std::str::FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn combination_test() {
        let hand = Hand::from_str("2H 3H 5H 9H KH").unwrap();
        assert!(
            matches!(
                Combination::try_from(hand).unwrap(),
                Combination::Flash(Suit::Hearts, Card { Suit::Hearts, 13 })
            )
        );
    }

    #[test]
    fn tie() {
        let black = Hand::from_str("2H 3D 5S 9C KD").unwrap();
        let white = Hand::from_str("2C 3H 4S 8C KH").unwrap();
        assert_eq!(black, white);
    }

    #[test]
    fn high_card() {
        let black = Hand::from_str("2H 3D 5S 9C KD").unwrap();
        let white = Hand::from_str("2C 3H 4S 8C AH").unwrap();
        /* Output: White wins - high card: Ace */
        assert!(white > black)
    }

    #[test]
    fn high_card2() {
        let black = Hand::from_str("2H 3D 5S 9C KD").unwrap();
        let white = Hand::from_str("2C 3H 4S 8C KH").unwrap();
        /* Output: Black wins - high card: 9 */
        assert!(black > white);
    }

    #[test]
    fn full_house() {
        let black = Hand::from_str("2H 4S 4C 2D 4H").unwrap();
        let white = Hand::from_str("2S 8S AS QS 3S").unwrap();
        /* Output: Black wins - full house */
        assert!(black > white);
    }
}
