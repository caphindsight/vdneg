use std::cmp::Ordering;
use cards::Card;
use cards::Rank;
use cards::Suit;

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

    if let Some(three_of_kind) = internal::detect_three_of_kind(cards) {
        three_of_kind
    } else if let Some(two_pair) = internal::detect_two_pair(cards) {
        two_pair
    } else if let Some(pair) = internal::detect_pair(cards) {
        pair
    } else {
        internal::detect_high_card(cards)
    }
}


mod internal {
    use super::Combo;
    use super::ComboRank;
    use cards::Card;
    use cards::Rank;
    use cards::Suit;

    struct RankBasket {
        rank: Rank,
        cards: Vec<Card>
    }

    fn split_by_rank(cards: &[Card]) -> Vec<RankBasket> {
        let mut copy: Vec<Card> = cards.to_vec();
        copy.sort_by_key(|c| c.rank);
        copy.reverse();
        
        let mut res = Vec::<RankBasket>::with_capacity(13);
        for c in &copy {
            let n = res.len();
            if n > 0 && res[n-1].rank == c.rank {
                res[n-1].cards.push(c.clone());
            } else {
                res.push(RankBasket{rank: c.rank, cards: vec![c.clone()]});
            }
        }
        res
    }

    fn find_basket(baskets: &mut Vec<RankBasket>, k: usize) -> Option<RankBasket> {
        let n = baskets.len();
        for i in 0..n {
            if baskets[i].cards.len() >= k {
                let basket_rank = baskets[i].rank;
                let mut res = Vec::<Card>::with_capacity(k);
                for i in 0..k {
                    let el = baskets[i].cards.pop().unwrap();
                    res.push(el);
                }
                if baskets[i].cards.len() == 0 {
                    baskets.remove(i);
                }
                return Some(RankBasket {rank: basket_rank, cards: res});
            }
        }
        None
    }

    fn join_baskets(baskets: &Vec<RankBasket>) -> Vec<Card> {
        let mut res = Vec::<Card>::new();
        for basket in baskets {
            for card in &basket.cards {
                res.push(card.clone());
            }
        }
        res
    }

    fn filter_by_suit(cards: &[Card], suit: Suit) -> Vec<Card> {
        let mut res = Vec::<Card>::with_capacity(cards.len());
        for c in cards {
            if c.suit == suit {
                res.push(c.clone());
            }
        }
        res
    }

    fn get_kickers(cards: &[Card], n: usize) -> Vec<Card> {
        assert!(n <= cards.len(), "not enough cards passed to get_kickers");

        let mut copy: Vec<Card> = cards.to_vec();
        copy.sort_by_key(|c| c.rank);
        copy.reverse();

        let mut res = Vec::<Card>::with_capacity(n);
        for i in 0..n {
            res.push(copy[i].clone());
        }
        res
    }

    // At least 5 cards are always present
    pub fn detect_high_card(cards: &[Card]) -> Combo {
        let kickers = get_kickers(cards, 5);
        Combo {
            combo_rank: ComboRank::HighCard,
            cards: [kickers[0], kickers[1], kickers[2], kickers[3], kickers[4]]
        }
    }

    // At least 5 cards are always present
    pub fn detect_pair(cards: &[Card]) -> Option<Combo> {
        let mut baskets = split_by_rank(cards);
        if let Some(pair) = find_basket(&mut baskets, 2) {
            let remaining = join_baskets(&baskets);
            let kickers = get_kickers(&remaining, 3);
            Some( Combo{
                combo_rank: ComboRank::Pair,
                cards: [pair.cards[0], pair.cards[1], kickers[0], kickers[1], kickers[2]]
            } )
        } else {
            None
        }
    }
    
    // At least 5 cards are always present
    pub fn detect_two_pair(cards: &[Card]) -> Option<Combo> {
        let mut baskets = split_by_rank(cards);
        if let Some(pair1) = find_basket(&mut baskets, 2) {
            if let Some(pair2) = find_basket(&mut baskets, 2) {
                let remaining = join_baskets(&baskets);
                let kickers = get_kickers(&remaining, 1);
                Some( Combo{
                    combo_rank: ComboRank::TwoPair,
                    cards: [pair1.cards[0], pair1.cards[1], pair2.cards[0], pair2.cards[1], kickers[0]]
                } )
            } else {
                None
            }
        } else {
            None
        }
    }

    // At least 5 cards are always present
    pub fn detect_three_of_kind(cards: &[Card]) -> Option<Combo> {
        let mut baskets = split_by_rank(cards);
        if let Some(three) = find_basket(&mut baskets, 3) {
            let remaining = join_baskets(&baskets);
            let kickers = get_kickers(&remaining, 2);
            Some( Combo{
                combo_rank: ComboRank::ThreeOfKind,
                cards: [three.cards[0], three.cards[1], three.cards[2], kickers[0], kickers[1]]
            } )
        } else {
            None
        }
    }
}

