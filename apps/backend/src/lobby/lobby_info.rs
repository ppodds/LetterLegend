#[derive(Debug, Clone)]
pub struct LobbyInfo {
    pub id: u32,
    pub max_players: u32,
    pub cur_players: u32,
}

impl PartialEq for LobbyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
