use std::sync::Arc;

use crate::lobby::{lobbies::Lobbies, lobby::Lobby};

include!(concat!(env!("OUT_DIR"), "/lobby.list.rs"));

impl LobbyInfos {
    pub async fn from_lobbies(lobbies: Arc<Lobbies>) -> Self {
        let mut lobby_infos = Vec::new();
        for lobby in lobbies.get_lobbies().await {
            lobby_infos.push(LobbyInfo::from_lobby(lobby.clone()).await);
        }
        Self { lobby_infos }
    }
}

impl LobbyInfo {
    pub async fn from_lobby(lobby: Arc<Lobby>) -> Self {
        Self {
            id: lobby.get_id(),
            max_players: lobby.get_max_players(),
            current_players: lobby.get_players().await.len() as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::player::Player;

    use super::*;
    use std::error::Error;

    #[tokio::test]
    async fn from_lobby_with_lobby_return_lobby_info() -> Result<(), Box<dyn Error + Send + Sync>> {
        let lobby = Arc::new(Lobby::new(0, 4));
        let lobby_info = LobbyInfo::from_lobby(lobby).await;
        assert_eq!(lobby_info.id, 0);
        assert_eq!(lobby_info.max_players, 4);
        assert_eq!(lobby_info.current_players, 0);
        Ok(())
    }

    #[tokio::test]
    async fn from_lobby_with_a_player_in_lobby_lobby_info_current_players_should_be_one(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let lobby = Arc::new(Lobby::new(0, 4));
        lobby
            .add_player(Arc::new(Player::new(0, String::from("test"))))
            .await?;
        let lobby_info = LobbyInfo::from_lobby(lobby).await;
        assert_eq!(lobby_info.current_players, 1);
        Ok(())
    }

    #[tokio::test]
    async fn from_lobbies_with_lobbies_return_lobby_infos(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let lobbies = Arc::new(Lobbies::new());
        let lobby_infos = LobbyInfos::from_lobbies(lobbies).await;
        assert_eq!(lobby_infos.lobby_infos.len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn from_lobbies_with_lobbies_has_one_lobby_return_lobby_infos(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let lobbies = Arc::new(Lobbies::new());
        lobbies.create_lobby(4).await;
        let lobby_infos = LobbyInfos::from_lobbies(lobbies).await;
        assert_eq!(lobby_infos.lobby_infos[0].id, 0);
        Ok(())
    }
}
