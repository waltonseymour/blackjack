use deckofcards::{Cards, Deck, Hand, Rank};

enum Outcome {
    Push,
    Win,
    Loss,
    BlackJack,
    Surrender,
}

#[derive(Debug)]
struct Value {
    // if is_soft is true, then (value - 10) is also possible due to an ace
    is_soft: bool,
    value: u8,
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

fn hand_value(hand: &Hand) -> Value {
    let mut total = 0;
    let mut is_soft = false;
    for card in hand.cards() {
        total += rank_to_u8(card.rank);
        if card.rank == Rank::Ace {
            is_soft = true;
        }
    }

    Value {
        is_soft,
        value: total,
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let mut hand = Hand::new();
    deck.deal_to_hand(&mut hand, 2);

    let value = hand_value(&hand);

    println!("{}", hand);
    println!("{:?}", value);
}
