use std::sync::Arc;

use std::sync::Mutex;

use crate::player::Player;

use super::card::Card;

#[derive(Debug)]
pub struct GamePlayer {
    cards: Mutex<Vec<Card>>,
    has_shuffled: Mutex<bool>,
    pub player: Arc<Player>,
}

impl PartialEq for GamePlayer {
    fn eq(&self, other: &Self) -> bool {
        self.player.id == other.player.id
    }
}

impl GamePlayer {
    pub fn new(player: Arc<Player>) -> Self {
        let cards = GamePlayer::generate_new_card();
        Self {
            cards: Mutex::new(cards),
            has_shuffled: Mutex::new(false),
            player,
        }
    }

    pub fn set_has_shuffled(&self, value: bool) {
        *self.has_shuffled.lock().unwrap() = value;
    }

    pub fn get_has_shuffled(&self) -> bool {
        *self.has_shuffled.lock().unwrap()
    }

    pub fn generate_new_card() -> Vec<Card> {
        let alphabet = (b'a'..=b'z') // Start as u8
            .map(|c| c as char) // Convert all to chars
            .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
            .collect::<Vec<_>>();
        let mut vowel = Vec::new();
        vowel.push('a');
        vowel.push('e');
        vowel.push('i');
        vowel.push('o');
        vowel.push('u');
        let mut cards: Vec<Card> = Vec::new();
        for _ in 0..3 {
            cards.push(Card::new(
                vowel[rand::random::<u8>() as usize % vowel.len()],
            ));
        }
        for _ in 3..8 {
            cards.push(Card::new(
                alphabet[rand::random::<u8>() as usize % alphabet.len()],
            ));
        }
        cards
    }

    pub fn get_new_card(&self) -> Vec<Card> {
        *self.cards.lock().unwrap() = GamePlayer::generate_new_card();
        *self.has_shuffled.lock().unwrap() = true;
        self.cards.lock().unwrap().clone()
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.cards.lock().unwrap().clone()
    }

    pub fn get_card(&self, index: usize) -> Card {
        self.cards.lock().unwrap()[index].clone()
    }

    pub fn take_card(&self, index: usize) -> Card {
        let mut cards = self.cards.lock().unwrap();
        let old_card = &cards[index];
        let mut card = Card::new(old_card.char);
        card.used = true;
        cards[index] = card.clone();
        card
    }

    pub fn return_cancel_card(&self, char: char) {
        let mut cards = self.cards.lock().unwrap();
        for i in 0..cards.len() {
            if cards[i].used && cards[i].char == char {
                cards[i].used = false;
                break;
            }
        }
    }
}
