use std::rc::Rc;

use anyhow::Result;

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
        fn should_return_none_with_invalid_user_input() {}

        #[test]
        fn should_produce_an_action_with_valid_user_input() {}
    }

    mod draw_page {
        use super::draw_page;

        #[test]
        fn should_render_user_actions() {}

        #[test]
        fn should_render_to_std_out() {}
    }
}
