mod probability;
use deckofcards::{Card, Cards, Deck, Hand, Rank};
use probability::BJProbability;

trait BJValue {
    fn value(&self) -> u8;

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
        println!("player hand: {} ({})", hand, hand.value());
        println!("{:?}", Outcome::BlackJack);
        return;
    }

    loop {
        println!(
            "{} ({}) bust probability: {:.3}",
            hand,
            hand.value(),
            hand.probability_of_bust()
        );

        if hand.is_bust() {
            break;
        }

        let action = get_user_action(&hand);

        match action {
            Action::Hit => {
                deck.deal_to_hand(&mut hand, 1);
            }
            Action::Stand => break,
            Action::DoubleDown => {
                deck.deal_to_hand(&mut hand, 1);
                break;
            }
            Action::Surrender => todo!(),
            Action::Split => todo!(),
        }
    }

    while !hand.is_bust() && !dealer_hand.is_bust() && dealer_hand.value() < 17 {
        deck.deal_to_hand(&mut dealer_hand, 1);
    }

    let outcome = get_outcome(&dealer_hand, &hand);

    println!("dealer hand: {} ({})", dealer_hand, dealer_hand.value());
    println!("player hand: {} ({})", hand, hand.value());

    println!("{:?}", outcome);
}

fn main() {
    let mut deck = build_deck();
    play_hand(&mut deck)
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
