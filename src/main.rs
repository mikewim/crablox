use log::{error, info};
use simplelog::*;
use std::env;
use std::io::{self, Write};
use std::{error::Error, fs};

pub mod parser;
pub mod scanner;
pub mod token;

fn main() -> Result<(), Box<dyn Error>> {
    parser::ast_printer_test();
    return Ok(());
    // initialize simple log
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => {
            let buffer = fs::read_to_string(&args[1]).unwrap();

            let result = scan(buffer);
            match result {
                Ok(tokens) => {
                    for token in tokens.iter() {
                        info!("{:?}", token);
                    }
                }
                Err(error) => error!("halp {:?}", error),
            }
        }
        _ => {
            error!("Usage: rlox[script]");
            std::process::exit(64);
        }
    }

    Ok(())
}

fn scan(src: String) -> Result<Vec<token::Token>, scanner::error::ScanError> {
    let mut sr = scanner::Scanner::new(src);
    sr.scan_tokens()
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        // if ctrl-d is pressed
        if buffer.is_empty() {
            break;
        }
        // Pop off newline at end
        buffer.pop();
        let result = scan(buffer);
        match result {
            Ok(tokens) => {
                for token in tokens.iter() {
                    info!("{:?}", token);
                }
            }
            Err(error) => error!("halp {:?}", error),
        }
    }
}

fn error(line: usize, msg: &str) {
    report(line, "", msg);
}

fn report(line: usize, where_at: &str, msg: &str) {
    error!("[line {}] Error: {} {}", line, where_at, msg);
}
