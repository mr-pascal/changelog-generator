pub fn combine_lines<S: Into<String>>(v: Vec<S>) -> String {
    v.into_iter()
        .map(|v| v.into())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_lines() -> Result<(), String> {
        // test with "String"
        let input = vec!["this".to_owned(), "is a".to_owned(), "test".to_owned()];
        let expected = "this\nis a\ntest".to_owned();
        let output = combine_lines(input);
        assert_eq!(output, expected);

        // test with "&str"
        let input2 = vec!["this", "is a", "test"];
        let expected2 = "this\nis a\ntest".to_owned();
        let output2 = combine_lines(input2);
        assert_eq!(output2, expected2);

        Ok(())
    }
}
