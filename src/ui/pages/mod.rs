use std::io::Write;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Ok, Result};
use itertools::Itertools;

use crate::{
    db::JiraDatabase,
    models::{Action, Epic},
};

mod page_helpers;
use page_helpers::*;

trait Page {
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn draw_page<W: Write>(&self, writer: &mut W) -> Result<()>;
}

struct HomePage {
    pub db: Arc<Mutex<JiraDatabase>>,
}

impl HomePage {
    fn new(db: Arc<Mutex<JiraDatabase>>) -> Self {
        HomePage { db }
    }
}

impl Page for HomePage {
    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        Ok(Some(Action::Exit))
    }
    fn draw_page<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer
            .write_all(b"-------------------------- EPICS ----------------------------------\n")?;
        writer
            .write_all(b" id |    name    |        description        |   status   |  count \n")?;

        let db = self.db.lock().map_err(|e| {
            anyhow!(format!(
                "Unable to access the page database. Original error: {}",
                e
            ))
        })?;
        let db = db
            .database
            .lock()
            .map_err(|e| anyhow!("Unable to lock the JiraDatabase database: {}", e))?;
        let current_state = db.read_db()?;

        let _lines: Result<()> = current_state
            .epics
            .iter()
            .sorted()
            .map(|(id, epic)| {
                let col1 = 4usize;
                let col2 = 10usize;
                let col3 = 25usize;
                let col4 = 9usize;
                let col5 = 5usize;

                match epic {
                    Epic {
                        name,
                        description,
                        status,
                        stories,
                    } => {
                        format!(
                            "{}|{}|{}|{}|{}",
                            get_column_string(id.to_string().as_str(), col1),
                            get_column_string(name, col2),
                            get_column_string(description, col3),
                            get_column_string(status.to_string().as_str(), col4),
                            get_column_string(stories.len().to_string().as_str(), col5),
                        )
                    }
                }
            })
            .map(|s| {
                writer
                    .write_all(s.as_bytes())
                    .map_err(|e| anyhow!(format!("Error writing epic: {}", e)))
            })
            .collect();

        writer.write_all(b"\n")?;
        writer.write_all(b"\n")?;

        writer.write_all(b"[q] quit  |  [c] create epic  |  [:id:] navigate to epic")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::db::{test_utils::MockDB, JiraDatabase};

    mod home_page {

        use std::{
            io::{self, Write},
            sync::{Arc, Mutex},
            thread,
        };

        use crate::models::{Epic, Status};

        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Arc::new(Mutex::new(JiraDatabase {
                database: Arc::new(Mutex::new(MockDB::new())),
            }));
            let home_page = HomePage::new(db);
            let mut buffer = Vec::<u8>::new();
            let result = home_page.draw_page(&mut buffer);

            assert!(result.is_ok())
        }

        #[test]
        fn should_render_to_std_out() {
            let db = Arc::new(Mutex::new(JiraDatabase {
                database: Arc::new(Mutex::new(MockDB::new())),
            }));
            let home_page = HomePage::new(db);

            let output = Arc::new(Mutex::new(Vec::<u8>::new()));
            let output_clone = Arc::clone(&output);

            let handle = thread::spawn(move || {
                let mut buffer = output_clone.lock().unwrap();
                let _ = home_page.draw_page(&mut *buffer);
            });

            handle.join().unwrap();

            let buffer = output.lock().unwrap();
            let result = String::from_utf8_lossy(&buffer);
            assert!(result
                .contains("-------------------------- EPICS ----------------------------------"));
        }

        #[test]
        fn should_render_persisted_epics() {
            let db = Arc::new(Mutex::new(JiraDatabase {
                database: Arc::new(Mutex::new(MockDB::new())),
            }));
            let epic = Epic::new(
                "Rainbows & Unicorns".to_owned(),
                "Things that make a great childhood".to_owned(),
            );

            let epic_id = db.lock().unwrap().create_epic(epic);
            assert!(epic_id.is_ok());
            let epic_id = epic_id.unwrap();

            let home_page = HomePage::new(db);
            let output = Arc::new(Mutex::new(Vec::<u8>::new()));
            let output_clone = Arc::clone(&output);

            let handle = thread::spawn(move || {
                let mut buffer = output_clone.lock().unwrap();
                // epic information in db should be printed here
                let _ = home_page.draw_page(&mut *buffer);
            });

            let thread_merge = handle.join();
            assert!(thread_merge.is_ok());

            let expected_id = epic_id.to_string();
            let expected_name = "Rainbows".to_string();
            let expected_status = Status::default().to_string();
            let expected_texts = vec![expected_id, expected_name, expected_status];
            let output = &output.lock().unwrap();
            let output_string = String::from_utf8_lossy(&output);

            let result: bool = expected_texts.iter().all(|s| output_string.contains(s));
            assert_eq!(result, true, "Not all expected content was found")
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            assert!(true, "Complete this stubbed test")
        }
    }
}
