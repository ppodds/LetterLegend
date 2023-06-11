include!(concat!(env!("OUT_DIR"), "/game.words.rs"));

impl From<&Vec<String>> for Words {
    fn from(value: &Vec<String>) -> Self {
        Self {
            words: value.iter().map(|word| word.clone()).collect(),
        }
    }
}
