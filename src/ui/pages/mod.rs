use anyhow::Result;

mod page_helpers;

use crate::models::Action;

fn handle_input() /*-> Result<Option<Action>>*/ {}

fn draw_page() {}

#[cfg(test)]
mod tests {
    use super::*;

    mod handle_input {
        use super::handle_input;

        #[test]
        fn should_fail_() {}

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
