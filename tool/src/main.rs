use simplelog::*;

mod generate_ast;

fn main() {
    // initialize simple log
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    generate_ast::generate();
}
