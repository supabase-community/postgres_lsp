pub enum CommandType {
    ExecuteStatement,
}

impl CommandType {
    pub const ALL: [CommandType; 1] = [CommandType::ExecuteStatement];

    pub fn id(&self) -> &str {
        match self {
            CommandType::ExecuteStatement => "executeStatement",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            CommandType::ExecuteStatement => "Execute Statement",
        }
    }

    pub fn from_id(s: &str) -> Option<CommandType> {
        match s {
            "executeStatement" => Some(CommandType::ExecuteStatement),
            _ => None,
        }
    }
}

pub trait Command {
    type ExecuteStatement;

    fn command_type() -> CommandType;
}
