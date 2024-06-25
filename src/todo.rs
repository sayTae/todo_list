use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub text: String,
    pub completed: bool,
}
