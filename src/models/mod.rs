// This file defines the data models for the todo AI assistant, describing the structure of todo items and user inputs.

pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(id: u32, title: String, description: String) -> Self {
        TodoItem {
            id,
            title,
            description,
            completed: false,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }
}

pub struct UserInput {
    pub input_text: String,
    pub timestamp: std::time::SystemTime,
}

impl UserInput {
    pub fn new(input_text: String) -> Self {
        UserInput {
            input_text,
            timestamp: std::time::SystemTime::now(),
        }
    }
}