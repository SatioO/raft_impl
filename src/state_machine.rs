use std::collections::HashMap;

#[derive(Debug)]
pub enum CommandKind {
    GetCommand,
    SetCommand,
}

impl CommandKind {
    fn index(&self) -> u8 {
        match self {
            CommandKind::GetCommand => 0,
            CommandKind::SetCommand => 1,
        }
    }

    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(CommandKind::GetCommand),
            1 => Some(CommandKind::SetCommand),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Command {
    pub kind: CommandKind,
    pub key: String,
    pub value: String,
}
/// Our state machine will have two operations: get a value from a key, and set a key to a value
impl Command {
    pub fn new(kind: CommandKind, key: String, value: Option<String>) -> Self {
        Command {
            kind,
            key,
            value: if let Some(val) = value {
                val
            } else {
                String::from("")
            },
        }
    }
}

pub fn encode_command(c: Command) -> Vec<u8> {
    let mut msg = Vec::new();

    // Write the kind
    msg.push(c.kind.index());

    // Write the length of the key (as u64 in little endian)
    let key_len = (c.key.len() as u64).to_le_bytes();
    msg.extend_from_slice(&key_len);

    // Write the key itself
    msg.extend_from_slice(c.key.as_bytes());

    // Write the length of the value (as u64 in little endian)
    let value_len = (c.value.len() as u64).to_le_bytes();
    msg.extend_from_slice(&value_len);

    // Write the value itself
    msg.extend_from_slice(c.value.as_bytes());

    // msg
    msg
}

fn decode_command(msg: &[u8]) -> Option<Command> {
    if msg.len() < 9 {
        return None;
    }

    let kind = CommandKind::from_u8(msg[0])?;

    let key_len = u64::from_le_bytes(msg[1..9].try_into().unwrap()) as usize;
    if msg.len() < 9 + key_len {
        return None;
    }

    let key = String::from_utf8(msg[9..9 + key_len].to_vec()).unwrap();

    let mut value = String::new();
    if let CommandKind::SetCommand = kind {
        if msg.len() < 9 + key_len + 8 {
            return None;
        }
        let value_len =
            u64::from_le_bytes(msg[9 + key_len..9 + key_len + 8].try_into().unwrap()) as usize;
        if msg.len() < 9 + key_len + 8 + value_len {
            return None;
        }
        value =
            String::from_utf8(msg[9 + key_len + 8..9 + key_len + 8 + value_len].to_vec()).unwrap();
    }

    Some(Command { kind, key, value })
}

#[derive(Debug, Default)]
pub struct StateMachine {
    pub db: HashMap<String, String>,
    pub server: usize,
}

impl StateMachine {
    fn apply(&mut self, cmd: &[u8]) -> Option<Vec<u8>> {
        let c = decode_command(cmd).unwrap();

        match c.kind {
            CommandKind::GetCommand => {
                if let Some(value) = self.db.get(&c.key) {
                    return Some(value.as_bytes().to_vec());
                } else {
                    return None;
                }
            }
            CommandKind::SetCommand => {
                self.db.insert(c.key.clone(), c.value.clone());
                return None;
            }
        }
    }
}
