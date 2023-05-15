use std::sync::Arc;

use std::sync::Mutex;

use crate::player::Player;

#[derive(Debug)]
pub struct GamePlayer {
    cards: Mutex<Vec<Option<char>>>,
    pub player: Arc<Player>,
}

impl PartialEq for GamePlayer {
    fn eq(&self, other: &Self) -> bool {
        self.player.id == other.player.id
    }
}

impl GamePlayer {
    pub fn new(player: Arc<Player>) -> Self {
        let alphabet = (b'a'..=b'z') // Start as u8
            .map(|c| c as char) // Convert all to chars
            .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
            .collect::<Vec<_>>();

        let mut cards = Vec::new();
        for _ in 0..8 {
            cards.push(Some(
                alphabet[rand::random::<u8>() as usize % alphabet.len()],
            ));
        }
        Self {
            cards: Mutex::new(cards),
            player,
        }
    }

    pub fn get_cards(&self) -> Vec<Option<char>> {
        self.cards.lock().unwrap().clone()
    }

    pub fn take_card(&self, index: usize) -> Option<char> {
        let card = self.cards.lock().unwrap()[index];
        self.cards.lock().unwrap()[index] = None;
        card
    }

    pub fn get_new_card(&self) {
        let alphabet = (b'a'..=b'z') // Start as u8
            .map(|c| c as char) // Convert all to chars
            .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
            .collect::<Vec<_>>();

        let mut cards = Vec::new();
        for _ in 0..8 {
            cards.push(Some(
                alphabet[rand::random::<u8>() as usize % alphabet.len()],
            ));
        }
        *self.cards.lock().unwrap() = cards;
    }
}
