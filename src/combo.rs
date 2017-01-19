use std::cmp::Ordering;
use cards::Card;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum ComboRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfKind,
    Straight,
    Flush,
    FullHouse,
    FourOfKind,
    StraightFlush,
    RoyalFlush,
}

impl ComboRank {
    pub fn to_string(self) -> String {
        match self {
            ComboRank::HighCard      => "high card",
            ComboRank::Pair          => "pair",
            ComboRank::TwoPair       => "two pair",
            ComboRank::ThreeOfKind   => "three of kind",
            ComboRank::Straight      => "straight",
            ComboRank::Flush         => "flush",
            ComboRank::FullHouse     => "full house",
            ComboRank::FourOfKind    => "four of kind",
            ComboRank::StraightFlush => "straight flush",
            ComboRank::RoyalFlush    => "royal flush",
        }.to_string()
    }
}

pub struct Combo {
    pub combo_rank: ComboRank,   // primary characteristic
    pub cards: [Card; 5],        // sorted in descending order
}

impl Combo {
    pub fn to_short_unicode_string(&self) -> String {
        format!("{}: {} {} {} {} {}",
                self.combo_rank.to_string(),
                self.cards[0].to_short_unicode_string(),
                self.cards[1].to_short_unicode_string(),
                self.cards[2].to_short_unicode_string(),
                self.cards[3].to_short_unicode_string(),
                self.cards[4].to_short_unicode_string())
    }
    
    pub fn to_long_unicode_string(&self) -> String {
        format!("{}: {} {} {} {} {}",
                self.combo_rank.to_string(),
                self.cards[0].to_long_unicode_string(),
                self.cards[1].to_long_unicode_string(),
                self.cards[2].to_long_unicode_string(),
                self.cards[3].to_long_unicode_string(),
                self.cards[4].to_long_unicode_string())
    }
    
    pub fn to_long_ascii_string(&self) -> String {
        format!("{}: {} {} {} {} {}",
                self.combo_rank.to_string(),
                self.cards[0].to_long_ascii_string(),
                self.cards[1].to_long_ascii_string(),
                self.cards[2].to_long_ascii_string(),
                self.cards[3].to_long_ascii_string(),
                self.cards[4].to_long_ascii_string())
    }
}

impl PartialEq for Combo {
    fn eq(&self, other: &Combo) -> bool {
        if self.combo_rank != other.combo_rank {
            return false;
        }
        for i in 0..5 {
            if self.cards[i].rank != other.cards[i].rank {
                return false;
            }
        }
        true
    }
}

impl Eq for Combo {}

impl Ord for Combo {
    fn cmp(&self, other: &Combo) -> Ordering {
        let rank_ordering = self.combo_rank.cmp(&other.combo_rank);
        if rank_ordering != Ordering::Equal {
            return rank_ordering;
        }
        for i in 0..5 {
            let card_rank_ordering = self.cards[i].rank.cmp(&other.cards[i].rank);
            if card_rank_ordering != Ordering::Equal {
                return card_rank_ordering;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Combo {
    fn partial_cmp(&self, other: &Combo) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


/// Detects the best combination that can be picked from a list of cards.
/// The list has to contain at least 5 cards!
pub fn detect_combo(cards: &[Card]) -> Combo {
    assert!(cards.len() >= 5, "less than 5 cards passed to detect_combo");
    let mut copy = cards.to_vec();
    detect_high_card(&mut copy)
}

// At least 5 cards are always present
fn detect_high_card(cards: &mut Vec<Card>) -> Combo {
    cards.sort_by_key(|c| c.rank);
    cards.reverse();
    Combo {
        combo_rank: ComboRank::HighCard,
        cards: [cards[0], cards[1], cards[2], cards[3], cards[4]]
    }
}

