#[derive(Debug, Clone)]
pub enum Operation {
    Connect,
    Disconnect,
    Heartbeat,
    CreateLobby,
}

impl TryFrom<u8> for Operation {
    type Error = Box<dyn std::error::Error + Send + Sync>;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operation::Connect),
            1 => Ok(Operation::Disconnect),
            2 => Ok(Operation::Heartbeat),
            3 => Ok(Operation::CreateLobby),
            _ => Err("invalid operation".into()),
        }
    }
}
