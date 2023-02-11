use deckofcards::Hand;

use crate::BJValue;

pub trait BJProbability {
    fn probability_of_bust(&self) -> f64;
}

impl BJProbability for Hand {
    fn probability_of_bust(&self) -> f64 {
        let val = self.value();
        if self.is_bust() || val == 21 {
            return 1.0;
        }

        let mut total = 0.0;
        for x in 1..11 {
            if val + x > 21 {
                total += probability_of_value(x);
            }
        }

        total
    }
}

// start off with general probablity, not taking in account cards already drawn
fn probability_of_value(val: u8) -> f64 {
    if val == 10 {
        return 4.0 / 13.0;
    }

    1.0 / 13.0
}

#[cfg(test)]
mod tests {
    use deckofcards::Card;

    use super::*;

    #[test]
    fn test_probability_of_bust() {
        // 15 value
        let cards = [
            Card::from_str("4C").unwrap(),
            Card::from_str("4D").unwrap(),
            Card::from_str("3C").unwrap(),
            Card::from_str("4S").unwrap(),
        ];

        let hand = Hand::from_cards(&cards);

        // 0.53 probability of bust
        assert!(hand.probability_of_bust() - 7.0 / 13.0 < 0.001);

        // 8 value
        let cards = [Card::from_str("4C").unwrap(), Card::from_str("4D").unwrap()];

        let hand = Hand::from_cards(&cards);

        assert_eq!(hand.probability_of_bust(), 0.0);
    }
}
