mod cards;
mod combo;

fn main() {
    let c = combo::Combo{
        combo_rank: combo::ComboRank::Flush,
        cards: [cards::ACE_OF_SPADES, cards::KING_OF_SPADES, cards::SEVEN_OF_SPADES,
            cards::FIVE_OF_SPADES, cards::DEUCE_OF_SPADES]
    };

    println!("{}", c.to_short_unicode_string());
}
