use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match nexus6::cli::parser::parse_args(&args) {
        Ok(cmd) => {
            if let Err(e) = nexus6::cli::runner::run(cmd) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!();
            nexus6::cli::runner::run(nexus6::cli::parser::CliCommand::Help).ok();
            process::exit(1);
        }
    }
}
