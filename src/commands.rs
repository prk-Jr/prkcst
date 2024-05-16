use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    alias: String,
    commands: Vec<String>,
}

impl Command {
    pub fn new(alias: String, commands: Vec<String>) -> Self {
        Self { alias, commands }
    }

    pub fn alias(&self) -> &str {
        &self.alias
    }

    pub fn commands(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.as_str()).collect()
    }
}
