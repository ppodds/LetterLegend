include!(concat!(env!("OUT_DIR"), "/game.card.rs"));

impl From<&Option<char>> for Card {
    fn from(value: &Option<char>) -> Self {
        Self {
            symbol: match value {
                Some(c) => Some(String::from(*c)),
                None => None,
            },
        }
    }
}
