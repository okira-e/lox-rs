#[cfg(test)]
mod test {
    use crate::lexing::lexer::Lexer;

     // Keep in mind that the lexer adds an EOF token to the end of the tokens vector.

    #[test]
    fn scan_tokens() {
        let input = "(*)";
        let mut lexer = Lexer::new(input);

        let (tokens, errors) = lexer.scan_tokens();
        assert_eq!(tokens.len(), 4);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comments() {
        let input = "() // This is a comment )";
        let mut lexer = Lexer::new(input);

        let (tokens, errors) = lexer.scan_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn whitespace() {
        let input = " ( ) ".into();
        let mut lexer = Lexer::new(input);

        let (tokens, errors) = lexer.scan_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn strings() {
        let input = "\"Hello, world!\"".into();
        let mut lexer = Lexer::new(input);

        let (tokens, errors) = lexer.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn unexpected_token() {
        let input = "(*^)";
        let mut lexer = Lexer::new(input);

        let (tokens, errors) = lexer.scan_tokens();

        for token in tokens {
            println!("{}", token.to_string());
        }

        assert_eq!(tokens.len(), 4);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn multiple_errors() {
        let input = "(*^) (+^) (^)";
        let mut lexer = Lexer::new(input);

        let (tokens, errors) = lexer.scan_tokens();

        assert_eq!(tokens.len(), 9);
        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn error_message() {
        let input = "(*^)";
        let mut lexer = Lexer::new(input);

        let (tokens, errors) = lexer.scan_tokens();
        assert_eq!(tokens.len(), 4);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, String::from("Unrecognized character \"^\" at line 1."));
    }

    // #[test]
    // fn unexpected_token_with_hint() {
    //     let input = "(*=)";
    //     let mut lexer = Lexer::new(input);
    //
    //     let (tokens, errors) = lexer.scan_tokens();
    //     assert_eq!(tokens.len(), 4);
    //     assert_eq!(errors.len(), 1);
    //     assert_eq!(errors[0].hint, Some(String::from("Did you mean \"*\"?")));
    // }
}
