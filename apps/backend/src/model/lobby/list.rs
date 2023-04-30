use std::sync::Arc;

use crate::lobby::lobby::Lobby;

include!(concat!(env!("OUT_DIR"), "/lobby.list.rs"));

impl From<Vec<Arc<Lobby>>> for LobbyInfos {
    fn from(lobbies: Vec<Arc<Lobby>>) -> Self {
        let mut lobby_infos = Vec::new();
        for lobby in lobbies {
            lobby_infos.push(LobbyInfo::from(lobby));
        }
        Self { lobby_infos }
    }
}

impl From<Arc<Lobby>> for LobbyInfo {
    fn from(lobby: Arc<Lobby>) -> Self {
        Self {
            id: lobby.get_id(),
            max_players: lobby.get_max_players(),
            current_players: lobby.get_players().len() as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lobby::lobbies::Lobbies, player::Player};

    use super::*;
    use std::error::Error;

    #[test]
    fn from_lobby_with_lobby_return_lobby_info() -> Result<(), Box<dyn Error + Send + Sync>> {
        let lobby = Arc::new(Lobby::new(
            0,
            4,
            Arc::new(Player::new(0, "test".to_string())),
        ));
        let lobby_info = LobbyInfo::from(lobby);
        assert_eq!(lobby_info.id, 0);
        assert_eq!(lobby_info.max_players, 4);
        assert_eq!(lobby_info.current_players, 1);
        Ok(())
    }

    #[test]
    fn from_lobby_with_a_player_in_lobby_lobby_info_current_players_should_be_one(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let player = Arc::new(Player::new(0, String::from("test")));
        let lobby = Arc::new(Lobby::new(0, 4, player.clone()));
        lobby.add_player(player.clone())?;
        let lobby_info = LobbyInfo::from(lobby);
        assert_eq!(lobby_info.current_players, 1);
        Ok(())
    }

    #[test]
    fn from_lobbies_with_lobbies_return_lobby_infos() -> Result<(), Box<dyn Error + Send + Sync>> {
        let lobbies = Arc::new(Lobbies::new());
        let lobby_infos = LobbyInfos::from(lobbies.get_lobbies());
        assert_eq!(lobby_infos.lobby_infos.len(), 0);
        Ok(())
    }

    #[test]
    fn from_lobbies_with_lobbies_has_one_lobby_return_lobby_infos(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let lobbies = Arc::new(Lobbies::new());
        lobbies.create_lobby(4, Arc::new(Player::new(0, "test".to_string())));
        let lobby_infos = LobbyInfos::from(lobbies.get_lobbies());
        assert_eq!(lobby_infos.lobby_infos[0].id, 0);
        Ok(())
    }
}
