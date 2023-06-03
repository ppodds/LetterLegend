use super::card::Card;

include!(concat!(env!("OUT_DIR"), "/game.cards.rs"));

impl From<&Vec<crate::game::card::Card>> for Cards {
    fn from(value: &Vec<crate::game::card::Card>) -> Self {
        Self {
            cards: value.iter().map(|x| Card::from(x)).collect(),
        }
    }
}
