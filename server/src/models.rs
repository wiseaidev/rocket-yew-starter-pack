use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub completed: bool,
    pub description: String,
    pub editing: bool,
}
