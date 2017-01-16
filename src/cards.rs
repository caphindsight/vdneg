extern crate rand;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Rank {
    Deuce,
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

impl Rank {
    fn to_string(self) -> String {
        match self {
            Rank::Deuce => "2",
            Rank::Three => "3",
            Rank::Four  => "4",
            Rank::Five  => "5",
            Rank::Six   => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine  => "9",
            Rank::Ten   => "t",
            Rank::Jack  => "J",
            Rank::Queen => "Q",
            Rank::King  => "K",
            Rank::Ace   => "A",
        }.to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn to_unicode_string(self) -> String {
        match self {
            Suit::Spades   => "\u{2660}",
            Suit::Hearts   => "\u{2665}",
            Suit::Diamonds => "\u{2666}",
            Suit::Clubs    => "\u{2663}",
        }.to_string()
    }

    pub fn to_ascii_string(self) -> String {
        match self {
            Suit::Spades   => "a",
            Suit::Hearts   => "b",
            Suit::Diamonds => "c",
            Suit::Clubs    => "d",
        }.to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn to_short_unicode_string(self) -> String {
        match self {
            DEUCE_OF_SPADES => "\u{1F0A2}",
            DEUCE_OF_HEARTS => "\u{1F0B2}",
            DEUCE_OF_DIAMONDS => "\u{1F0C2}",
            DEUCE_OF_CLUBS => "\u{1F0D2}",
            
            THREE_OF_SPADES => "\u{1F0A3}",
            THREE_OF_HEARTS => "\u{1F0B3}",
            THREE_OF_DIAMONDS => "\u{1F0C3}",
            THREE_OF_CLUBS => "\u{1F0D3}",

            FOUR_OF_SPADES => "\u{1F0A4}",
            FOUR_OF_HEARTS => "\u{1F0B4}",
            FOUR_OF_DIAMONDS => "\u{1F0C4}",
            FOUR_OF_CLUBS => "\u{1F0D4}",

            FIVE_OF_SPADES => "\u{1F0A5}",
            FIVE_OF_HEARTS => "\u{1F0B5}",
            FIVE_OF_DIAMONDS => "\u{1F0C5}",
            FIVE_OF_CLUBS => "\u{1F0D5}",

            SIX_OF_SPADES => "\u{1F0A6}",
            SIX_OF_HEARTS => "\u{1F0B6}",
            SIX_OF_DIAMONDS => "\u{1F0C6}",
            SIX_OF_CLUBS => "\u{1F0D6}",

            SEVEN_OF_SPADES => "\u{1F0A7}",
            SEVEN_OF_HEARTS => "\u{1F0B7}",
            SEVEN_OF_DIAMONDS => "\u{1F0C7}",
            SEVEN_OF_CLUBS => "\u{1F0D7}",

            EIGHT_OF_SPADES => "\u{1F0A8}",
            EIGHT_OF_HEARTS => "\u{1F0B8}",
            EIGHT_OF_DIAMONDS => "\u{1F0C8}",
            EIGHT_OF_CLUBS => "\u{1F0D8}",
            
            NINE_OF_SPADES => "\u{1F0A9}",
            NINE_OF_HEARTS => "\u{1F0B9}",
            NINE_OF_DIAMONDS => "\u{1F0C9}",
            NINE_OF_CLUBS => "\u{1F0D9}",

            TEN_OF_SPADES => "\u{1F0AA}",
            TEN_OF_HEARTS => "\u{1F0BA}",
            TEN_OF_DIAMONDS => "\u{1F0CA}",
            TEN_OF_CLUBS => "\u{1F0DA}",
            
            JACK_OF_SPADES => "\u{1F0AB}",
            JACK_OF_HEARTS => "\u{1F0BB}",
            JACK_OF_DIAMONDS => "\u{1F0CB}",
            JACK_OF_CLUBS => "\u{1F0DB}",
            
            QUEEN_OF_SPADES => "\u{1F0AD}",
            QUEEN_OF_HEARTS => "\u{1F0BD}",
            QUEEN_OF_DIAMONDS => "\u{1F0CD}",
            QUEEN_OF_CLUBS => "\u{1F0DD}",
            
            KING_OF_SPADES => "\u{1F0AE}",
            KING_OF_HEARTS => "\u{1F0BE}",
            KING_OF_DIAMONDS => "\u{1F0CE}",
            KING_OF_CLUBS => "\u{1F0DE}",
            
            ACE_OF_SPADES => "\u{1F0A1}",
            ACE_OF_HEARTS => "\u{1F0B1}",
            ACE_OF_DIAMONDS => "\u{1F0C1}",
            ACE_OF_CLUBS => "\u{1F0D1}",
        }.to_string()
    }

    pub fn to_long_unicode_string(self) -> String {
        let mut res = self.rank.to_string();
        res.push_str(&self.suit.to_unicode_string());
        res
    }

    pub fn to_long_ascii_string(self) -> String {
        let mut res = self.rank.to_string();
        res.push_str(&self.suit.to_ascii_string());
        res
    }
}

pub const DEUCE_OF_SPADES: Card = Card {rank: Rank::Deuce, suit: Suit::Spades};
pub const DEUCE_OF_HEARTS: Card = Card {rank: Rank::Deuce, suit: Suit::Hearts};
pub const DEUCE_OF_DIAMONDS: Card = Card {rank: Rank::Deuce, suit: Suit::Diamonds};
pub const DEUCE_OF_CLUBS: Card = Card {rank: Rank::Deuce, suit: Suit::Clubs};

pub const THREE_OF_SPADES: Card = Card {rank: Rank::Three, suit: Suit::Spades};
pub const THREE_OF_HEARTS: Card = Card {rank: Rank::Three, suit: Suit::Hearts};
pub const THREE_OF_DIAMONDS: Card = Card {rank: Rank::Three, suit: Suit::Diamonds};
pub const THREE_OF_CLUBS: Card = Card {rank: Rank::Three, suit: Suit::Clubs};

pub const FOUR_OF_SPADES: Card = Card {rank: Rank::Four, suit: Suit::Spades};
pub const FOUR_OF_HEARTS: Card = Card {rank: Rank::Four, suit: Suit::Hearts};
pub const FOUR_OF_DIAMONDS: Card = Card {rank: Rank::Four, suit: Suit::Diamonds};
pub const FOUR_OF_CLUBS: Card = Card {rank: Rank::Four, suit: Suit::Clubs};

pub const FIVE_OF_SPADES: Card = Card {rank: Rank::Five, suit: Suit::Spades};
pub const FIVE_OF_HEARTS: Card = Card {rank: Rank::Five, suit: Suit::Hearts};
pub const FIVE_OF_DIAMONDS: Card = Card {rank: Rank::Five, suit: Suit::Diamonds};
pub const FIVE_OF_CLUBS: Card = Card {rank: Rank::Five, suit: Suit::Clubs};

pub const SIX_OF_SPADES: Card = Card {rank: Rank::Six, suit: Suit::Spades};
pub const SIX_OF_HEARTS: Card = Card {rank: Rank::Six, suit: Suit::Hearts};
pub const SIX_OF_DIAMONDS: Card = Card {rank: Rank::Six, suit: Suit::Diamonds};
pub const SIX_OF_CLUBS: Card = Card {rank: Rank::Six, suit: Suit::Clubs};

pub const SEVEN_OF_SPADES: Card = Card {rank: Rank::Seven, suit: Suit::Spades};
pub const SEVEN_OF_HEARTS: Card = Card {rank: Rank::Seven, suit: Suit::Hearts};
pub const SEVEN_OF_DIAMONDS: Card = Card {rank: Rank::Seven, suit: Suit::Diamonds};
pub const SEVEN_OF_CLUBS: Card = Card {rank: Rank::Seven, suit: Suit::Clubs};

pub const EIGHT_OF_SPADES: Card = Card {rank: Rank::Eight, suit: Suit::Spades};
pub const EIGHT_OF_HEARTS: Card = Card {rank: Rank::Eight, suit: Suit::Hearts};
pub const EIGHT_OF_DIAMONDS: Card = Card {rank: Rank::Eight, suit: Suit::Diamonds};
pub const EIGHT_OF_CLUBS: Card = Card {rank: Rank::Eight, suit: Suit::Clubs};

pub const NINE_OF_SPADES: Card = Card {rank: Rank::Nine, suit: Suit::Spades};
pub const NINE_OF_HEARTS: Card = Card {rank: Rank::Nine, suit: Suit::Hearts};
pub const NINE_OF_DIAMONDS: Card = Card {rank: Rank::Nine, suit: Suit::Diamonds};
pub const NINE_OF_CLUBS: Card = Card {rank: Rank::Nine, suit: Suit::Clubs};

pub const TEN_OF_SPADES: Card = Card {rank: Rank::Ten, suit: Suit::Spades};
pub const TEN_OF_HEARTS: Card = Card {rank: Rank::Ten, suit: Suit::Hearts};
pub const TEN_OF_DIAMONDS: Card = Card {rank: Rank::Ten, suit: Suit::Diamonds};
pub const TEN_OF_CLUBS: Card = Card {rank: Rank::Ten, suit: Suit::Clubs};

pub const JACK_OF_SPADES: Card = Card {rank: Rank::Jack, suit: Suit::Spades};
pub const JACK_OF_HEARTS: Card = Card {rank: Rank::Jack, suit: Suit::Hearts};
pub const JACK_OF_DIAMONDS: Card = Card {rank: Rank::Jack, suit: Suit::Diamonds};
pub const JACK_OF_CLUBS: Card = Card {rank: Rank::Jack, suit: Suit::Clubs};

pub const QUEEN_OF_SPADES: Card = Card {rank: Rank::Queen, suit: Suit::Spades};
pub const QUEEN_OF_HEARTS: Card = Card {rank: Rank::Queen, suit: Suit::Hearts};
pub const QUEEN_OF_DIAMONDS: Card = Card {rank: Rank::Queen, suit: Suit::Diamonds};
pub const QUEEN_OF_CLUBS: Card = Card {rank: Rank::Queen, suit: Suit::Clubs};

pub const KING_OF_SPADES: Card = Card {rank: Rank::King, suit: Suit::Spades};
pub const KING_OF_HEARTS: Card = Card {rank: Rank::King, suit: Suit::Hearts};
pub const KING_OF_DIAMONDS: Card = Card {rank: Rank::King, suit: Suit::Diamonds};
pub const KING_OF_CLUBS: Card = Card {rank: Rank::King, suit: Suit::Clubs};

pub const ACE_OF_SPADES: Card = Card {rank: Rank::Ace, suit: Suit::Spades};
pub const ACE_OF_HEARTS: Card = Card {rank: Rank::Ace, suit: Suit::Hearts};
pub const ACE_OF_DIAMONDS: Card = Card {rank: Rank::Ace, suit: Suit::Diamonds};
pub const ACE_OF_CLUBS: Card = Card {rank: Rank::Ace, suit: Suit::Clubs};


pub fn deck_unshuffled() -> Vec<Card> { vec![
    ACE_OF_SPADES,
    DEUCE_OF_SPADES,
    THREE_OF_SPADES,
    FOUR_OF_SPADES,
    FIVE_OF_SPADES,
    SIX_OF_SPADES,
    SEVEN_OF_SPADES,
    EIGHT_OF_SPADES,
    NINE_OF_SPADES,
    TEN_OF_SPADES,
    JACK_OF_SPADES,
    QUEEN_OF_SPADES,
    KING_OF_SPADES,

    ACE_OF_HEARTS,
    DEUCE_OF_HEARTS,
    THREE_OF_HEARTS,
    FOUR_OF_HEARTS,
    FIVE_OF_HEARTS,
    SIX_OF_HEARTS,
    SEVEN_OF_HEARTS,
    EIGHT_OF_HEARTS,
    NINE_OF_HEARTS,
    TEN_OF_HEARTS,
    JACK_OF_HEARTS,
    QUEEN_OF_HEARTS,
    KING_OF_HEARTS,
    
    ACE_OF_DIAMONDS,
    DEUCE_OF_DIAMONDS,
    THREE_OF_DIAMONDS,
    FOUR_OF_DIAMONDS,
    FIVE_OF_DIAMONDS,
    SIX_OF_DIAMONDS,
    SEVEN_OF_DIAMONDS,
    EIGHT_OF_DIAMONDS,
    NINE_OF_DIAMONDS,
    TEN_OF_DIAMONDS,
    JACK_OF_DIAMONDS,
    QUEEN_OF_DIAMONDS,
    KING_OF_DIAMONDS,
    
    ACE_OF_CLUBS,
    DEUCE_OF_CLUBS,
    THREE_OF_CLUBS,
    FOUR_OF_CLUBS,
    FIVE_OF_CLUBS,
    SIX_OF_CLUBS,
    SEVEN_OF_CLUBS,
    EIGHT_OF_CLUBS,
    NINE_OF_CLUBS,
    TEN_OF_CLUBS,
    JACK_OF_CLUBS,
    QUEEN_OF_CLUBS,
    KING_OF_CLUBS,
] }

pub fn deck_shuffled() -> Vec<Card> {
    use cards::rand::Rng;
    let mut a = deck_unshuffled();
    rand::thread_rng().shuffle(&mut a);
    a
}

