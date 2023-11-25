use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u8,
    pub completed: bool,
    pub description: String,
    pub editing: bool,
}
