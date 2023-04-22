#[derive(Debug, Clone)]
pub enum Operation {
    Connect,
    Disconnect,
    Heartbeat,
    CreateLobby,
    JoinLobby,
    QuitLobby,
    ListLobby,
}

impl TryFrom<u8> for Operation {
    type Error = Box<dyn std::error::Error + Send + Sync>;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operation::Connect),
            1 => Ok(Operation::Disconnect),
            2 => Ok(Operation::Heartbeat),
            3 => Ok(Operation::CreateLobby),
            4 => Ok(Operation::JoinLobby),
            5 => Ok(Operation::QuitLobby),
            6 => Ok(Operation::ListLobby),
            _ => Err("invalid operation".into()),
        }
    }
}
