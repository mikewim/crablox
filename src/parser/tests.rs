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
    let expr = Binary {
        left: Box::new(Expr::Unary(Unary {
            operator: Token::new(TokenType::Minus),
            right: Box::new(Expr::Literal(Literal(LiteralType::Integer(123)))),
        })),
        operator: Token::new(TokenType::Star),
        right: Box::new(Expr::Grouping(Grouping {
            expression: Box::new(Expr::Literal(Literal(LiteralType::Float(45.67)))),
        })),
    };
    let mut sr = Parser::new(string.clone());
    let result = sr.scan_tokens();
    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_eq!(tokens[0].token_type, TokenType::LoxString(string));

    assert_eq!(tokens[tokens.len() - 1].token_type, TokenType::EOF);
}
