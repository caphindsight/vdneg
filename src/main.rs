mod cards;

fn main() {
    let deck = cards::deck_shuffled();
    println!("{} {}", deck[0].to_short_unicode_string(), deck[1].to_short_unicode_string());
}
