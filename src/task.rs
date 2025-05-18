use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub desc: String,
    pub is_done: bool,
}

impl Default for Task {
    fn default() -> Task {
        Task {
            title: "no title".to_string(),
            desc: "no description".to_string(),
            is_done: false,
        }
    }
}
