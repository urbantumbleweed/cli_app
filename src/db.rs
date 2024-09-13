use std::{fs, io};

use anyhow::{Context, Error, Result};
use serde_json::json;

use crate::models::{DBState, Epic};

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let db_str = fs::read_to_string(&self.file_path)?;
        let db: DBState = serde_json::from_str(&db_str)?;
        Ok(db)
    }
    fn write_db(&self, db_state: &DBState) -> Result<()> {
        let db_ser = json!(db_state);
        fs::write(&self.file_path, db_ser.to_string())
            .context("failed to write the db to the filesystem")
    }
}

pub struct JiraDatabase {
    database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }
    pub fn create_epic(&mut self, epic: Epic) -> Result<u32> {
        let mut current_state = self.read_db()?;
        let id = current_state.last_item_id + 1;
        let _ = &current_state.epics.insert(id, epic.clone());
        current_state.last_item_id = id;
        let _ = self.database.write_db(&current_state);
        Ok(id)
    }
    pub fn read<U>(&self, id: &U) -> Result<Epic> {
        todo!("Implement the ability to query the db for an entry based on the hashmap id")
    }
    pub fn delete<V>(&mut self, id: u32) -> Result<Epic> {
        todo!("Implement removing database items by id")
    }
}
pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;

    pub struct MockDB {
        last_written_state: RefCell<DBState>,
    }

    impl MockDB {
        pub fn new() -> Self {
            MockDB {
                last_written_state: RefCell::new(DBState {
                    last_item_id: 0,
                    epics: HashMap::new(),
                    stories: HashMap::new(),
                }),
            }
        }
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }
        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;

            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use std::collections::HashMap;
        use std::hash::Hash;
        use std::io::Write;

        use test_utils::MockDB;

        use crate::models::{Epic, Status, Story};

        use super::JSONFileDatabase;

        use super::*;

        #[test]
        fn create_a_new_epic() {
            let mut db = JiraDatabase {
                database: Box::new(MockDB::new()),
            };
            let epic = Epic::new("".to_owned(), "".to_owned());

            let result = db.create_epic(epic.clone());

            assert_eq!(result.is_ok(), true, "The result was an error");

            let id = result.unwrap();
            let db_state = db.read_db().unwrap();

            let expected_id = 1;

            assert_eq!(id, expected_id, "The resulting Id is not incremented");
            assert_eq!(
                db_state.last_item_id, expected_id,
                "The last_item_id should match the incremented id"
            );
            assert_eq!(db_state.epics.get(&id), Some(&epic))
        }

        #[test]
        fn create_story_should_error_with_invalid_epic_id() {
            let db = JiraDatabase {
                database: Box::new(MockDB::new()),
            };
            let story = Story::new("Sample text".to_owned(), "description text".to_owned());
            let invalid_id: u32 = 100;
            let result: Result<u32> = db.create_story(story, invalid_id);

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase {
                file_path: "INVALID PATH".to_owned(),
            };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let invalid_json_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", invalid_json_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile to str")
                    .to_string(),
            };

            let result = db.read_db();
            assert_eq!(result.is_err(), true)
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let valid_json_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {}}"#;
            write!(tmpfile, "{}", valid_json_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let result = db.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let valid_json_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {}}"#;
            write!(tmpfile, "{}", valid_json_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile to str")
                    .to_string(),
            };

            let story = Story {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
            };

            let epic = Epic {
                name: "epic 1".to_owned(),
                description: "epic 1".to_owned(),
                status: Status::Open,
                stories: vec![2],
            };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState {
                last_item_id: 2,
                epics,
                stories,
            };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);

            assert_eq!(read_result, state);
        }
    }
}
