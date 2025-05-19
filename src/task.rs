use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            .field("title", &self.title)
            .field("desc", &self.desc)
            .field("is_done", &self.is_done)
            .finish()
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_default_task(){
        let task: Task = Task::default();
        assert_eq!(task.title, "no title");
        assert!(!task.is_done);
    }
}
