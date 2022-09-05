use deckofcards::{Card, Cards, Deck, Hand, Rank};

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

impl Value {
    fn is_bust(self) -> bool {
        self.value > 21
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
    Surrender,
}

fn get_user_action() -> Action {
    println!("H for hit\nS for stand");
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

    match user_input.trim() {
        "H" => Action::Hit,
        "S" => Action::Stand,
        _ => get_user_action(),
    }
}

fn main() {
    let mut deck = build_deck();

    let mut dealer_hand = Hand::new();

    deck.deal_to_hand(&mut dealer_hand, 1);

    println!("dealer showing: {}", dealer_hand);

    let mut hand = Hand::new();
    deck.deal_to_hand(&mut hand, 2);

    loop {
        let value = hand_value(&hand);
        println!("{}", hand);

        if value.is_bust() {
            break;
        }

        let action = get_user_action();

        match action {
            Action::Hit => {
                deck.deal_to_hand(&mut hand, 1);
            }
            Action::Stand => todo!(),
            Action::DoubleDown => todo!(),
            Action::Surrender => todo!(),
        }
    }
}
