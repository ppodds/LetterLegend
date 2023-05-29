use std::error::Error;

use crate::frame::RequestData;

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

impl TryFrom<&RequestData> for Operation {
    type Error = Box<dyn Error + Send + Sync>;
    fn try_from(value: &RequestData) -> Result<Self, Self::Error> {
        match value {
            RequestData::Connect(_) => Ok(Operation::Connect),
            RequestData::Disconnect => Ok(Operation::Disconnect),
            RequestData::Heartbeat => Ok(Operation::Heartbeat),
            RequestData::CreateLobby(_) => Ok(Operation::CreateLobby),
            RequestData::JoinLobby(_) => Ok(Operation::JoinLobby),
            RequestData::QuitLobby => Ok(Operation::QuitLobby),
            RequestData::ListLobby => Ok(Operation::ListLobby),
            RequestData::Ready => Ok(Operation::Ready),
            RequestData::StartGame => Ok(Operation::StartGame),
            RequestData::SetTile(_) => Ok(Operation::SetTile),
            RequestData::FinishTurn => Ok(Operation::FinishTurn),
            RequestData::GetNewCard => Ok(Operation::GetNewCard),
            // _ => Err("invalid request".into()),
        }
    }
}
