use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum Status {
    #[default]
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Open => f.write_str("OPEN"),
            Self::InProgress => f.write_str("IN PROGRESS"),
            Self::Resolved => f.write_str("RESOLVED"),
            Self::Closed => f.write_str("CLOSED"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod status {
        use super::Status;

        #[test]
        fn status_should_default_to_open() {
            let s = Status::default();
            assert_eq!(s, Status::Open)
        }

        #[test]
        fn status_has_a_to_string_method() {
            let statuses: Vec<Status> = vec![
                Status::Open,
                Status::InProgress,
                Status::Resolved,
                Status::Closed,
            ];
            let expected_strings = vec!["OPEN", "IN PROGRESS", "RESOLVED", "CLOSED"];
            let strings_match: bool = statuses
                .iter()
                .zip(expected_strings.iter())
                .map(|(actual, expected)| {
                    let actual_string = actual.to_string();
                    if &actual_string == expected {
                        Ok(actual_string)
                    } else {
                        Err(format!("{} and {} do not match", &actual_string, &expected))
                    }
                })
                .all(|result| result.is_ok());

            assert_eq!(strings_match, true)
        }
    }
}
