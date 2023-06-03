#[derive(Debug, Clone)]

pub struct Card {
    pub char: char,
    pub used: bool,
}

impl Card {
    pub fn new(char: char) -> Self {
        Self { char, used: false }
    }
}
