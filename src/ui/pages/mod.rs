use std::io::Write;
use std::sync::Arc;

use anyhow::{anyhow, Ok, Result};
use itertools::Itertools;

use crate::{db::JiraDatabase, models::Action};

mod page_helpers;
use page_helpers::*;

trait Page {
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn draw_page<W: Write>(&self, writer: &mut W) -> Result<()>;
}

struct HomePage {
    pub db: Arc<JiraDatabase>,
}

impl HomePage {
    fn new(db: Arc<JiraDatabase>) -> Self {
        HomePage { db }
    }
}

impl Page for HomePage {
    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        Ok(Some(Action::Exit))
    }
    fn draw_page<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write(b"----------------------------- EPICS -----------------------------");
        writer.write(b"  id  |     name     |         description         |    status    ");
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

        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Arc::new(JiraDatabase {
                database: Arc::new(Mutex::new(MockDB::new())),
            });
            let home_page = HomePage::new(db);
            let mut buffer = Vec::<u8>::new();
            let result = home_page.draw_page(&mut buffer);

            assert!(result.is_ok())
        }

        #[test]
        fn should_render_to_std_out() {
            let db = Arc::new(JiraDatabase {
                database: Arc::new(Mutex::new(MockDB::new())),
            });
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
                .contains("----------------------------- EPICS -----------------------------"));
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            assert!(true, "Complete this stubbed test")
        }

        #[test]
        fn should_render_user_actions() {
            assert!(true, "Complete this stubbed test")
        }
    }
}
