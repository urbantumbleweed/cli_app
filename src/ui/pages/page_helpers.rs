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
            let width = 5usize;

            let expected = String::from("hello...");
            let result = get_column_string(&text, width);

            assert_eq!(result, expected);
        }
    }
}
