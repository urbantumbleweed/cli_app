use std::{fs, io};

use anyhow::{anyhow, Context, Error, Result};
use serde_json::json;

use crate::models::{DBState, Epic, Story};

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
    pub fn create_story(&mut self, story: Story, epic_id: u32) -> Result<u32> {
        let mut current_state = self.database.read_db().context("Error fetching database")?;
        let new_story_id = current_state.last_item_id + 1;
        current_state.stories.insert(new_story_id, story.clone());
        let mut epic = current_state
            .epics
            .get(&epic_id)
            .context("Error getting epic")?
            .clone();
        epic.stories.push(new_story_id);
        current_state.last_item_id = new_story_id;
        let _ = self.database.write_db(&current_state);

        Ok(new_story_id)
    }
    pub fn read<U>(&self, id: &U) -> Result<Epic> {
        todo!("Implement the ability to query the db for an entry based on the hashmap id")
    }
    pub fn delete_epic(&mut self, id: u32) -> Result<Epic> {
        let mut current_state = self.database.read_db().context("Error fetching database")?;
        match current_state.epics.remove(&id) {
            Some(deleted_epic) => {
                let _ = self.database.write_db(&current_state);
                Ok(deleted_epic)
            }
            None => Err(anyhow!(
                "The epic with Id: {} could not be removed because it is invalid or does not exist",
                &id
            )),
        }
    }
    pub fn delete_story(&mut self, epic_id: u32, story_id: u32) -> Result<Story> {
        let mut db_state = self.read_db()?;
        let mut epic = db_state
            .epics
            .get(&epic_id)
            .with_context(|| "Epic was not fetched")?;
        let story = db_state
            .stories
            .get(&story_id)
            .with_context(|| "Story was not fetched")?;
        let deleted_story = db_state.stories.remove(&story_id);

        let _ = self.database.write_db(&db_state)?;

        deleted_story.with_context(|| format!("No story with the id: {} was found", &story_id))
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

    use test_utils::MockDB;

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
        let mut db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let story = Story::new("Sample text".to_owned(), "description text".to_owned());
        let invalid_id: u32 = 100;
        let result: Result<u32> = db.create_story(story, invalid_id);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn create_story_should_work() {
        let mut db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("Key Project".to_owned(), "Attach some stories".to_owned());
        let story = Story::new(
            "New story".to_owned(),
            "A really important new story".to_owned(),
        );

        let result_epic_id = db.create_epic(epic);
        assert_eq!(result_epic_id.is_ok(), true);

        let epic_id = result_epic_id.unwrap();

        let result = db.create_story(story.clone(), epic_id);
        assert_eq!(
            result.is_ok(),
            true,
            "Calling `.create_story` resulted in an error"
        );

        let expected_id = 2u32;
        let id = result.unwrap();
        assert_eq!(
            id, expected_id,
            "The `id` of the added story didn't match the expected value"
        );

        let persisted_story = db
            .database
            .read_db()
            .unwrap()
            .stories
            .get(&id)
            .unwrap()
            .clone();
        assert_eq!(
            persisted_story, story,
            "The story fetched does not match the story persisted"
        );
    }

    #[test]
    fn delete_epic_should_error_with_invalid_epic_id() {
        let mut db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let new_epic = db.create_epic(epic.clone());
        assert_eq!(new_epic.is_ok(), true);

        let invalid_epic_id = 99;
        let result = db.delete_epic(invalid_epic_id);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_epic_should_delete_existing_epic() {
        let mut db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let result_epic_id = db.create_epic(epic.clone());

        assert_eq!(result_epic_id.is_ok(), true);

        let id = result_epic_id.unwrap();
        let returned_epic = db.delete_epic(id).unwrap();

        let db_state = db.database.read_db().unwrap();
        let not_found = db_state.epics.get(&id);
        assert_eq!(not_found, None);
        assert_eq!(returned_epic, epic);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let mut db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let res_epic_id = db.create_epic(epic);
        assert!(res_epic_id.is_ok(), "Epic did not persist to db.");

        let epic_id = res_epic_id.unwrap();
        let res_story_id = db.create_story(story, epic_id);
        assert!(res_story_id.is_ok(), "The story was not created in DB");

        let invalid_epic_id = 999;
        assert_ne!(
            epic_id, invalid_epic_id,
            "The epic ID should not be invalid"
        );

        let result = db.delete_story(invalid_epic_id, res_story_id.unwrap());
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let mut db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());
        let res_epic_id = db.create_epic(epic);
        assert!(res_epic_id.is_ok());

        let epic_id = res_epic_id.unwrap();
        let res_story_id = db.create_story(story, epic_id);
        assert!(res_story_id.is_ok());

        let invalid_story_id = 999;
        assert_ne!(invalid_story_id, res_story_id.unwrap());

        let result = db.delete_story(epic_id, invalid_story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_work() {
        let mut db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());
        let res_epic_id = db.create_epic(epic);
        assert!(res_epic_id.is_ok(), "Unable to write new epic");

        let epic_id = res_epic_id.unwrap();
        let res_story_id = db.create_story(story.clone(), epic_id);
        assert!(res_story_id.is_ok(), "Unable to write new story");

        let story_id = res_story_id.unwrap();

        let res_deleted_story = db.delete_story(epic_id, story_id);
        assert!(res_deleted_story.is_ok(), "Unable to delete story");
        assert_eq!(
            res_deleted_story.unwrap(),
            story,
            "The deleted story does not match was was added"
        );

        let mut current_state = db.read_db().unwrap();
        let not_found_story = current_state.stories.get(&story_id);
        assert_eq!(
            not_found_story, None,
            "Requesting the deleted story should be None"
        );

        let epic_stories_len = current_state
            .epics
            .get_mut(&epic_id)
            .map(|epic| epic.stories.len());

        let expected_epic_story_len = 0usize;
        assert_eq!(epic_stories_len.unwrap(), expected_epic_story_len);
    }

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use crate::models::{Epic, Status, Story};

        use super::JSONFileDatabase;

        use super::*;

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
