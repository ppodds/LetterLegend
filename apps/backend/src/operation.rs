#[derive(Debug, Clone)]
pub enum Operation {
    Disconnect,
    Heartbeat,
}

impl TryFrom<u8> for Operation {
    type Error = Box<dyn std::error::Error + Send + Sync>;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operation::Disconnect),
            1 => Ok(Operation::Heartbeat),
            _ => Err("invalid operation".into()),
        }
    }
}
