use super::card::Card;

include!(concat!(env!("OUT_DIR"), "/game.hand_card.rs"));

impl From<&crate::game::card::Card> for HandCard {
    fn from(value: &crate::game::card::Card) -> Self {
        Self {
            card: match value.used {
                true => None,
                false => Some(Card::from(value)),
            },
        }
    }
}
