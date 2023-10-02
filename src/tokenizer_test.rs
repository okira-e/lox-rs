#[cfg(test)]
mod scanning {
    use crate::tokenizer::Tokenizer;

     // Keep in mind that the tokenizer adds an EOF token to the end of the tokens vector.

    #[test]
    fn scan_tokens() {
        let input = "(*)";
        let mut tokenizer = Tokenizer::new(input);

        let (tokens, errors) = tokenizer.scan_tokens();
        assert_eq!(tokens.len(), 4);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn comments() {
        let input = "() // This is a comment )";
        let mut tokenizer = Tokenizer::new(input);

        let (tokens, errors) = tokenizer.scan_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn whitespace() {
        let input = " ( ) ".into();
        let mut tokenizer = Tokenizer::new(input);

        let (tokens, errors) = tokenizer.scan_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn strings() {
        let input = "\"Hello, world!\"".into();
        let mut tokenizer = Tokenizer::new(input);

        let (tokens, errors) = tokenizer.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn unterminated_string() {
        let input = "\"Hello, world!))";
        let mut tokenizer = Tokenizer::new(input);

        let (tokens, errors) = tokenizer.scan_tokens();

        assert_eq!(tokens.len(), 1);
        assert_eq!(errors.len(), 1);
    }
}

// #[cfg(test)]
// mod errors {
//     use crate::tokenizer::Tokenizer;
//
//     #[test]
//     fn unexpected_token() {
//         let input = "(*^)";
//         let mut tokenizer = Tokenizer::new(input);
//
//         let (tokens, errors) = tokenizer.scan_tokens();
//
//         assert_eq!(tokens.len(), 4);
//         assert_eq!(errors.len(), 1);
//     }
//
//     #[test]
//     fn multiple_errors() {
//         let input = "(*^) (+^) (^)";
//         let mut tokenizer = Tokenizer::new(input);
//
//         let (tokens, errors) = tokenizer.scan_tokens();
//
//         assert_eq!(tokens.len(), 9);
//         assert_eq!(errors.len(), 3);
//     }
//
//     #[test]
//     fn error_message() {
//         let input = "(*^)";
//         let mut tokenizer = Tokenizer::new(input);
//
//         let (tokens, errors) = tokenizer.scan_tokens();
//         assert_eq!(tokens.len(), 4);
//         assert_eq!(errors.len(), 1);
//         assert_eq!(errors[0].message, String::from("Unrecognized character \"^\" at line 1."));
//     }
// }