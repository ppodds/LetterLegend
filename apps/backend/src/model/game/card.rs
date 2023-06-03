include!(concat!(env!("OUT_DIR"), "/game.card.rs"));

impl From<&crate::game::card::Card> for Card {
    fn from(value: &crate::game::card::Card) -> Self {
        Self {
            symbol: String::from(value.char),
        }
    }
}
