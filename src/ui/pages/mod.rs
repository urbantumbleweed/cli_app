use std::rc::Rc;

use anyhow::{anyhow, Ok, Result};
use itertools::Itertools;

use crate::{db::JiraDatabase, models::Action};

mod page_helpers;
use page_helpers::*;

trait Page {
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn draw_page(&self) -> Result<()>;
}

struct HomePage {
    pub db: Rc<JiraDatabase>,
}

impl HomePage {
    fn new(db: Rc<JiraDatabase>) -> Self {
        HomePage { db }
    }
}

impl Page for HomePage {
    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        Ok(Some(Action::Exit))
    }
    fn draw_page(&self) -> Result<()> {
        println!("----------------------------- EPICS -----------------------------");
        println!("  id  |     name     |         description         |    status    ");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::db::{test_utils::MockDB, JiraDatabase};

    mod home_page {

        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });
            let home_page = HomePage::new(db);
            let result = home_page.draw_page();

            assert!(result.is_ok())
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            assert!(true, "Complete this stubbed test")
        }

        #[test]
        fn should_render_user_actions() {
            assert!(true, "Complete this stubbed test")
        }

        #[test]
        fn should_render_to_std_out() {
            assert!(true, "Complete this stubbed test")
        }
    }
}
