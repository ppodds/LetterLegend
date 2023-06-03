include!(concat!(env!("OUT_DIR"), "/game.card.rs"));

impl From<&crate::game::card::Card> for Card {
    fn from(value: &crate::game::card::Card) -> Self {
        Self {
            symbol: match value.used {
                true => Some(String::from(value.char)),
                false => None,
            },
        }
    }
}
