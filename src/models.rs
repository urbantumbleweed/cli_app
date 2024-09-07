use std::collections::HashMap;

#[derive(Default)]
pub enum Status {
    #[default]
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<i32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic {
            name,
            description,
            status: Status::default(),
            stories: Vec::new(),
        }
    }
}

pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story {
            name,
            description,
            status: Status::default(),
        }
    }
}

pub struct DBState {
    pub last_item_id: i32,
    pub epics: HashMap<i32, Epic>,
    pub stories: HashMap<i32, Story>,
}
