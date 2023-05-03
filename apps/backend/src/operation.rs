use std::error::Error;

use crate::frame::Request;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Operation {
    Connect,
    Disconnect,
    Heartbeat,
    CreateLobby,
    JoinLobby,
    QuitLobby,
    ListLobby,
    Ready,
    StartGame,
    SetTile,
    FinishTurn,
    GetNewCard,
}

impl TryFrom<u8> for Operation {
    type Error = Box<dyn Error + Send + Sync>;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operation::Connect),
            1 => Ok(Operation::Disconnect),
            2 => Ok(Operation::Heartbeat),
            3 => Ok(Operation::CreateLobby),
            4 => Ok(Operation::JoinLobby),
            5 => Ok(Operation::QuitLobby),
            6 => Ok(Operation::ListLobby),
            7 => Ok(Operation::Ready),
            8 => Ok(Operation::StartGame),
            9 => Ok(Operation::SetTile),
            10 => Ok(Operation::FinishTurn),
            11 => Ok(Operation::GetNewCard),
            _ => Err("invalid operation".into()),
        }
    }
}

impl TryFrom<&Request> for Operation {
    type Error = Box<dyn Error + Send + Sync>;
    fn try_from(value: &Request) -> Result<Self, Self::Error> {
        match value {
            Request::Connect(_) => Ok(Operation::Connect),
            Request::Disconnect => Ok(Operation::Disconnect),
            Request::Heartbeat => Ok(Operation::Heartbeat),
            Request::CreateLobby(_) => Ok(Operation::CreateLobby),
            Request::JoinLobby(_) => Ok(Operation::JoinLobby),
            Request::QuitLobby => Ok(Operation::QuitLobby),
            Request::ListLobby => Ok(Operation::ListLobby),
            Request::Ready => Ok(Operation::Ready),
            Request::StartGame => Ok(Operation::StartGame),
            Request::SetTile(_) => Ok(Operation::SetTile),
            Request::FinishTurn => Ok(Operation::FinishTurn),
            Request::GetNewCard => Ok(Operation::GetNewCard),
            // _ => Err("invalid request".into()),
        }
    }
}
