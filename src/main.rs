mod probability;
use deckofcards::{Card, Cards, Deck, Hand, Rank};
use probability::BJProbability;

trait BJValue {
    fn value(&self) -> u8;

    fn is_soft(&self) -> bool;

    fn is_bust(&self) -> bool {
        self.value() > 21
    }

    fn is_blackjack(&self) -> bool;
}

impl BJValue for Hand {
    fn value(&self) -> u8 {
        let mut total = 0;
        let mut is_soft = false;
        for card in self.cards() {
            total += rank_to_u8(card.rank);
            if card.rank == Rank::Ace {
                is_soft = true;
            }
        }

        if total > 21 && is_soft {
            total - 10
        } else {
            total
        }
    }

    fn is_soft(&self) -> bool {
        let mut total = 0;
        let mut is_soft = false;
        for card in self.cards() {
            total += rank_to_u8(card.rank);
            if card.rank == Rank::Ace {
                is_soft = true;
            }
        }

        if total > 21 && is_soft {
            return false;
        }

        is_soft
    }

    fn is_blackjack(&self) -> bool {
        self.value() == 21 && self.cards.len() == 2
    }
}

fn rank_to_u8(rank: Rank) -> u8 {
    match rank {
        Rank::Two => 2,
        Rank::Three => 3,
        Rank::Four => 4,
        Rank::Five => 5,
        Rank::Six => 6,
        Rank::Seven => 7,
        Rank::Eight => 8,
        Rank::Nine => 9,
        Rank::Ace => 11,
        _ => 10,
    }
}

fn build_deck() -> Deck {
    // four decks
    let cards = [
        Card::all_cards(),
        Card::all_cards(),
        Card::all_cards(),
        Card::all_cards(),
    ]
    .concat();

    let mut deck = Deck::from_cards(&cards);
    deck.shuffle();

    deck
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Action {
    Hit,
    Stand,
    DoubleDown,
    Split,
    Surrender,
}

fn can_split(player_hand: &Hand) -> bool {
    player_hand.len() == 2
        && player_hand.cards.get(0).unwrap().rank == player_hand.cards.get(1).unwrap().rank
        // can't split on aces
        && player_hand.cards.get(0).unwrap().rank != Rank::Ace
}

fn get_user_action(player_hand: &Hand) -> Action {
    if player_hand.len() == 2 {
        if (can_split(player_hand)) {
            println!("H for hit\nS for stand\nD for Double down\nP for Split\nR for Surrender");
        } else {
            println!("H for hit\nS for stand\nD for Double down\nR for Surrender");
        }
    } else {
        println!("H for hit\nS for stand");
    }

    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    match user_input.trim() {
        "H" => Action::Hit,
        "S" => Action::Stand,
        "D" => Action::DoubleDown,
        "R" => Action::Surrender,
        _ => get_user_action(player_hand),
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Outcome {
    Push,
    Win,
    Loss,
    BlackJack,
    Surrender,
}

fn get_outcome(dealer_hand: &Hand, player_hand: &Hand) -> Outcome {
    if player_hand.is_bust() {
        return Outcome::Loss;
    } else if dealer_hand.is_bust() {
        return Outcome::Win;
    }

    if dealer_hand.value() > player_hand.value() {
        Outcome::Loss
    } else if player_hand.value() > dealer_hand.value() {
        Outcome::Win
    } else {
        Outcome::Push
    }
}

fn play_hand(deck: &mut Deck) {
    let mut dealer_hand = Hand::new();

    deck.deal_to_hand(&mut dealer_hand, 1);

    println!("dealer showing: {} ({})", dealer_hand, dealer_hand.value());

    let mut hand = Hand::new();
    deck.deal_to_hand(&mut hand, 2);

    if hand.is_blackjack() {
        // check for dealer blackjack
        if (dealer_hand.value() >= 10) {
            deck.deal_to_hand(&mut dealer_hand, 1);

            if (dealer_hand.is_blackjack()) {
                return;
            }
        }

        println!("player hand: {} ({})", hand, hand.value());
        println!("{:?}", Outcome::BlackJack);
        return;
    }

    println!("{} ({} hard: {})", hand, hand.value(), !hand.is_soft());

    let correct_action = get_correct_action(&hand, &dealer_hand);

    let user_action = get_user_action(&hand);

    if user_action == correct_action {
        println!("\ncorrect\n");
    } else {
        println!("\nincorrect! correct action was: {:?}\n", correct_action);
    }
}

fn main() {
    let mut deck = build_deck();
    loop {
        play_hand(&mut deck);
    }
}

fn get_correct_action(player_hand: &Hand, dealer_hand: &Hand) -> Action {
    let dealer_value = dealer_hand.value();

    let player_value = player_hand.value();

    if !player_hand.is_soft() {
        // hard totals

        // hard 17+ stand always
        if player_value >= 17 {
            return Action::Stand;
        }

        // hard 13-17
        if player_value >= 13 && player_value < 17 {
            if dealer_value < 7 {
                return Action::Stand;
            }
        }

        // hard 12
        if player_value == 12 {
            if dealer_value >= 4 && dealer_value <= 6 {
                return Action::Stand;
            }
        }

        // hard 11, 10, 9
        if player_value == 11
            || (player_value == 10 && dealer_value < 10)
            || (player_value == 9 && dealer_value >= 3 && dealer_value <= 6)
        {
            return Action::DoubleDown;
        }

        //surrender

        if player_value == 15 && dealer_value == 10 {
            return Action::Surrender;
        }

        if player_value == 16 && dealer_value >= 9 {
            return Action::Surrender;
        }
    } else {
        // soft totals

        // soft 19/20 stands
        if player_value >= 19 {
            return Action::Stand;
        }

        if player_value == 18 {
            if dealer_value == 7 && dealer_value == 8 {
                return Action::Stand;
            }

            if dealer_value < 7 {
                return Action::DoubleDown;
            }
        }

        if player_value == 17 && dealer_value >= 3 && dealer_value <= 6 {
            return Action::DoubleDown;
        }

        if (player_value == 16 || player_value == 15) && dealer_value >= 4 && dealer_value <= 6 {
            return Action::DoubleDown;
        }

        if (player_value == 13 || player_value == 14) && dealer_value >= 5 && dealer_value <= 6 {
            return Action::DoubleDown;
        }
    }

    Action::Hit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_value() {
        // 4C,4D,2H,3C,AC
        let cards = [
            Card::from_str("4C").unwrap(),
            Card::from_str("4D").unwrap(),
            Card::from_str("3C").unwrap(),
            Card::from_str("AC").unwrap(),
        ];

        let hand = Hand::from_cards(&cards);

        assert_eq!(hand.is_bust(), false);
        assert_eq!(hand.value(), 12);
    }

    #[test]
    fn test_outome() {
        // 2S,6C,2S,JS
        let player_hand = Hand::from_cards(&[
            Card::from_str("2S").unwrap(),
            Card::from_str("6C").unwrap(),
            Card::from_str("2S").unwrap(),
            Card::from_str("JS").unwrap(),
        ]);

        // 5C,KC,KD
        let dealer_hand = Hand::from_cards(&[
            Card::from_str("5C").unwrap(),
            Card::from_str("KC").unwrap(),
            Card::from_str("KD").unwrap(),
        ]);

        let outcome = get_outcome(&dealer_hand, &player_hand);

        assert_eq!(outcome, Outcome::Win);
    }

    #[test]
    fn test_blakjack() {
        // 2S,6C,2S,JS
        let player_hand = Hand::from_cards(&[
            Card::from_str("2S").unwrap(),
            Card::from_str("6C").unwrap(),
            Card::from_str("2S").unwrap(),
            Card::from_str("JS").unwrap(),
        ]);

        // 5C,KC,KD
        let dealer_hand = Hand::from_cards(&[
            Card::from_str("5C").unwrap(),
            Card::from_str("KC").unwrap(),
            Card::from_str("KD").unwrap(),
        ]);

        let outcome = get_outcome(&dealer_hand, &player_hand);

        assert_eq!(outcome, Outcome::Win);
    }
}
