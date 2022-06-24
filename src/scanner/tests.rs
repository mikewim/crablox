use super::*;
use simplelog::*;

// This could be handy to debug a failing test
#[allow(dead_code)]
fn init_log() {
    // initialize simple log
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap_or(());
}

#[test]
fn read_string_happy_path() {
    let string = String::from(
        r#""Hello, my friend. Let's chat.
        How are you today?
        I'm good, thanks!
    ""#,
    );
    let mut sr = Scanner::new(string.clone());
    let result = sr.scan_tokens();
    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_eq!(
        tokens[0].token_type,
        TokenType::Literal(LiteralType::LoxString(string))
    );

    assert_eq!(tokens[tokens.len() - 1].token_type, TokenType::EOF);
}

#[test]
fn read_string_errors_on_unterminated() {
    let string = String::from(
        r#""Hello, my friend. Let's chat.
        How are you today?
        I'm good, thanks!
    "#,
    );
    let mut sr = Scanner::new(string.clone());
    let result = sr.scan_tokens();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err, error::ScanError::UnterminatedString);
}

#[test]
fn read_integer_happy_path() {
    let numbers = vec!["123", "3456098", "98", "1"];
    let mut sr = Scanner::new(numbers.join(" "));
    let result = sr.scan_tokens();
    assert!(result.is_ok());

    let tokens = result.unwrap();
    for (i, str_num) in numbers.iter().enumerate() {
        let num = str_num.parse::<isize>().unwrap();
        assert_eq!(
            tokens[i].token_type,
            TokenType::Literal(LiteralType::Integer(num))
        );
    }

    assert_eq!(tokens[tokens.len() - 1].token_type, TokenType::EOF);
}

#[test]
fn read_float_happy_path() {
    let numbers = vec!["123.9345", "3456098.4", "98.0000001", "0.45"];
    let mut sr = Scanner::new(numbers.join(" "));
    let result = sr.scan_tokens();
    assert!(result.is_ok());

    let tokens = result.unwrap();
    for (i, str_num) in numbers.iter().enumerate() {
        let num = str_num.parse::<f64>().unwrap();
        assert_eq!(
            tokens[i].token_type,
            TokenType::Literal(LiteralType::Float(num))
        );
    }

    assert_eq!(tokens[tokens.len() - 1].token_type, TokenType::EOF);
}

#[test]
fn read_float_cannot_end_with_decimal() {
    let mut sr = Scanner::new(String::from(r#"42."#));
    let result = sr.scan_tokens();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err, error::ScanError::NotValidNumber);
}

#[test]
fn uknown_token_error() {
    let mut sr = Scanner::new(String::from(r#"@"#));
    let result = sr.scan_tokens();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err, error::ScanError::UnknownToken);
}
