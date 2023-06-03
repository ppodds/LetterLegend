use crate::model::game::hand_card::HandCard;

include!(concat!(env!("OUT_DIR"), "/game.cards.rs"));

impl From<&Vec<crate::game::card::Card>> for Cards {
    fn from(value: &Vec<crate::game::card::Card>) -> Self {
        Self {
            cards: value.iter().map(|x| HandCard::from(x)).collect(),
        }
    }
}
