use std::{
    cmp::{Ordering, PartialEq, PartialOrd},
    convert::TryFrom,
    str::FromStr,
};

#[derive(Debug, PartialEq)]
enum Value {
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
    Ace,
}

impl From<&Value> for u8 {
    fn from(v: &Value) -> Self {
        match v {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            Value::Ace => 14,
        }
    }

}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        u8::from(self).partial_cmp(&u8::from(other))
    }
}

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

#[derive(Debug)]
enum Combination {
    HighCard(Card) ,
    Pair(Value),
    TwoPairs(Value, Value),
    Three(Value),
    Straitgh(Value),
    Flash(Card),
    FullHouse(Value, Value),
    Four(Value),
    StraightFlush(Suit, Card),
}

impl From<&Combination> for u8 {
    fn from(combination: &Combination) -> u8 {
        match combination {
            Combination::HighCard(_) => 1,
            Combination::Pair(_) => 2,
            Combination::TwoPairs(_, _) => 3,
            Combination::Three(_) => 4,
            Combination::Straitgh(_) => 5,
            Combination::Flash(_) => 6,
            Combination::FullHouse(_, _) => 7,
            Combination::Four(_) => 8,
            Combination::StraightFlush(_, _) => 9,
        }
    }
}

impl PartialEq for Combination {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Combination {
    fn partial_cmp(&self, other: &Combination) -> Option<Ordering> {
        match (self, other)
        {
            (Self::HighCard(self_card), Self::HighCard(other_card)) => self_card.value.partial_cmp(&other_card.value),
            (Self::Pair(self_value), Self::Pair(other_value)) => self_value.partial_cmp(&other_value),
            (Self::TwoPairs(self_pair1, self_pair2), Self::TwoPairs(other_pair1, other_pair2)) if self_pair1.eq(other_pair1) => self_pair2.partial_cmp(&other_pair2),
            (Self::TwoPairs(self_pair1, _self_pair2), Self::TwoPairs(other_pair1, _other_pair2)) => self_pair1.partial_cmp(other_pair1),
            (Self::Three(self_value), Self::Three(other_value)) => self_value.partial_cmp(&other_value),
            (Self::Straitgh(self_value), Self::Straitgh(other_value)) => self_value.partial_cmp(&other_value),
            (Self::Flash(self_card), Self::Flash(other_card)) => self_card.value.partial_cmp(&other_card.value),
            (Self::FullHouse(self_third, self_pair), Self::FullHouse(other_third, other_pair)) if self_third.eq(&other_third) => self_pair.partial_cmp(&other_pair),
            (Self::FullHouse(self_third, _), Self::FullHouse(other_third, _)) => self_third.partial_cmp(&other_third),
            (Self::Four(self_value), Self::Four(other_value)) => self_value.partial_cmp(&other_value),
            (Self::StraightFlush(_, self_value), Self::StraightFlush(_, other_value)) => self_value.value.partial_cmp(&other_value.value),
            (left, right) => u8::from(left).partial_cmp(&u8::from(right)),
        }
    }
}

impl From<&Hand> for Combination {
    fn from(_hand: &Hand) -> Self {
        todo!()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl PartialEq for Hand {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const H2: Card = Card {
        suit: Suit::Hearts,
        value: Value::Two,
    };
    const H3: Card = Card {
        suit: Suit::Hearts,
        value: Value::Three,
    };
    const H4: Card = Card {
        suit: Suit::Hearts,
        value: Value::Four,
    };
    const H5: Card = Card {
        suit: Suit::Hearts,
        value: Value::Five,
    };
    const H6: Card = Card {
        suit: Suit::Hearts,
        value: Value::Six,
    };
    const H7: Card = Card {
        suit: Suit::Hearts,
        value: Value::Seven,
    };
    const H8: Card = Card {
        suit: Suit::Hearts,
        value: Value::Eight,
    };
    const H9: Card = Card {
        suit: Suit::Hearts,
        value: Value::Nine,
    };
    const H10: Card = Card {
        suit: Suit::Hearts,
        value: Value::Ten,
    };
    const HJ: Card = Card {
        suit: Suit::Hearts,
        value: Value::Jack,
    };
    const HQ: Card = Card {
        suit: Suit::Hearts,
        value: Value::Queen,
    };
    const HK: Card = Card {
        suit: Suit::Hearts,
        value: Value::King,
    };
    const HA: Card = Card {
        suit: Suit::Hearts,
        value: Value::Ace,
    };

    const C2: Card = Card {
        suit: Suit::Clubs,
        value: Value::Two,
    };
    const C3: Card = Card {
        suit: Suit::Clubs,
        value: Value::Three,
    };
    const C4: Card = Card {
        suit: Suit::Clubs,
        value: Value::Four,
    };
    const C5: Card = Card {
        suit: Suit::Clubs,
        value: Value::Five,
    };
    const C6: Card = Card {
        suit: Suit::Clubs,
        value: Value::Six,
    };
    const C7: Card = Card {
        suit: Suit::Clubs,
        value: Value::Seven,
    };
    const C8: Card = Card {
        suit: Suit::Clubs,
        value: Value::Eight,
    };
    const C9: Card = Card {
        suit: Suit::Clubs,
        value: Value::Nine,
    };
    const C10: Card = Card {
        suit: Suit::Clubs,
        value: Value::Ten,
    };
    const CJ: Card = Card {
        suit: Suit::Clubs,
        value: Value::Jack,
    };
    const CQ: Card = Card {
        suit: Suit::Clubs,
        value: Value::Queen,
    };
    const CK: Card = Card {
        suit: Suit::Clubs,
        value: Value::King,
    };
    const CA: Card = Card {
        suit: Suit::Clubs,
        value: Value::Ace,
    };

    const D2: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Two,
    };
    const D3: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Three,
    };
    const D4: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Four,
    };
    const D5: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Five,
    };
    const D6: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Six,
    };
    const D7: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Seven,
    };
    const D8: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Eight,
    };
    const D9: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Nine,
    };
    const D10: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Ten,
    };
    const DJ: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Jack,
    };
    const DQ: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Queen,
    };
    const DK: Card = Card {
        suit: Suit::Diamonds,
        value: Value::King,
    };
    const DA: Card = Card {
        suit: Suit::Diamonds,
        value: Value::Ace,
    };

    const S2: Card = Card {
        suit: Suit::Spades,
        value: Value::Two,
    };
    const S3: Card = Card {
        suit: Suit::Spades,
        value: Value::Three,
    };
    const S4: Card = Card {
        suit: Suit::Spades,
        value: Value::Four,
    };
    const S5: Card = Card {
        suit: Suit::Spades,
        value: Value::Five,
    };
    const S6: Card = Card {
        suit: Suit::Spades,
        value: Value::Six,
    };
    const S7: Card = Card {
        suit: Suit::Spades,
        value: Value::Seven,
    };
    const S8: Card = Card {
        suit: Suit::Spades,
        value: Value::Eight,
    };
    const S9: Card = Card {
        suit: Suit::Spades,
        value: Value::Nine,
    };
    const S10: Card = Card {
        suit: Suit::Spades,
        value: Value::Ten,
    };
    const SJ: Card = Card {
        suit: Suit::Spades,
        value: Value::Jack,
    };
    const SQ: Card = Card {
        suit: Suit::Spades,
        value: Value::Queen,
    };
    const SK: Card = Card {
        suit: Suit::Spades,
        value: Value::King,
    };
    const SA: Card = Card {
        suit: Suit::Spades,
        value: Value::Ace,
    };

    mod combination {
        use super::*;
        use crate::{Card, Combination};

        #[test]
        fn hight_card() {
            assert!(Combination::HighCard(SQ) < Combination::HighCard(SA));
            assert!(Combination::HighCard(H2) < Combination::HighCard(H3));
            assert!(Combination::HighCard(SA) == Combination::HighCard(HA));
            assert!(Combination::HighCard(SA) > Combination::HighCard(SK));
        }

        #[test]
        fn pair() {
            assert!(Combination::Pair(Value::Two) < Combination::Pair(Value::King));
        }

        #[test]
        fn two_pair() {
            assert!(Combination::TwoPairs(Value::Queen, Value::Seven) > Combination::TwoPairs(Value::Queen, Value::Two));
            assert!(Combination::TwoPairs(Value::Queen, Value::Seven) > Combination::TwoPairs(Value::Six, Value::Seven));
        }

        #[test]
        fn three() {
            assert!(Combination::Three(Value::Queen) > Combination::Three(Value::Seven));
        }

        #[test]
        fn straitgh() {
            assert!(Combination::Straitgh(Value::Queen) > Combination::Straitgh(Value::Seven));
        }

         #[test]
        fn flash() {
            assert!(Combination::Flash(SA) > Combination::Flash(DK));
            assert_eq!(Combination::Flash(SA), Combination::Flash(DA));
        }

         #[test]
        fn full_house() {
            assert!(Combination::FullHouse(Value::Queen, Value::Seven) > Combination::FullHouse(Value::Queen, Value::Two));
            assert!(Combination::FullHouse(Value::Queen, Value::Seven) > Combination::FullHouse(Value::Six, Value::Seven));
        }

         #[test]
        fn four() {
            assert!(Combination::Four(Value::Seven) > Combination::Four(Value::Six));
        }
    }

    #[test]
    fn flash_test() {
        let combination = Combination::from(&Hand([H2, H3, H5, H9, H10]));
        assert_eq!(combination, Combination::Flash(H10));
    }

    #[test]
    fn tie() {
        let black = Combination::from(&Hand([H2, D3, S5, C9, DK]));
        let white = Combination::from(&Hand([C2, H3, SK, C8, HK]));
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
        let black = Combination::from(&Hand([H2, D3, D3, S5, C9]));
        let white = Combination::from(&Hand([C2, H3, S4, C8, HK]));
        /* Output: Black wins - high card: 9 */
        assert!(black > white);
    }

    #[test]
    fn full_house() {
        let black = Combination::from(&Hand([H3, S4, C4, D2, H4]));
        let white = Combination::from(&Hand([S2, S8, SA, SQ, S3]));
        /* Output: Black wins - full house */
        assert!(black > white);
    }
}
