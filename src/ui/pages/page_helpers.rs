use ellipse::Ellipse;

fn get_column_string(text: &str, width: usize) -> String {
    text.truncate_ellipse(width).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    mod get_column_string {
        use super::*;

        #[test]
        fn should_ellipsify_a_string_to_given_width() {
            let text = "hello world";
            let width0 = 0usize;
            let width1 = 1usize;
            let width2 = 2usize;
            let width3 = 3usize;
            let width4 = 4usize;
            let width5 = 5usize;

            assert_eq!(get_column_string(text, width0), "".to_owned());
            assert_eq!(get_column_string(text, width1), "h...".to_owned());
            assert_eq!(get_column_string(text, width2), "he...".to_owned());
            assert_eq!(get_column_string(text, width3), "hel...".to_owned());
            assert_eq!(get_column_string(text, width4), "hell...".to_owned());
            assert_eq!(get_column_string(text, width5), "hello...".to_owned());
        }

        #[test]
        fn should_produce_empty_string_with_width_of_zero() {
            let text = "yo";
            let width = 0usize;

            let expected = String::from("");
            let result = get_column_string(text, width);

            assert_eq!(result, expected)
        }

        #[test]
        fn should_display_complete_text_if_smaller_than_width() {
            let text = "kiss me";
            let width = 20usize;

            let expected = String::from("kiss me");
            let result = get_column_string(text, width);

            assert_eq!(result, expected)
        }
    }
}
