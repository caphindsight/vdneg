mod cards;
mod combo;

fn main() {
    let deck = [
        cards::ACE_OF_SPADES,
        cards::SEVEN_OF_DIAMONDS,
        cards::KING_OF_CLUBS,
        cards::SEVEN_OF_SPADES,
        cards::SEVEN_OF_HEARTS,
    ];
    let res = combo::detect_combo(&deck);
    println!("{}", res.to_short_unicode_string());
}
