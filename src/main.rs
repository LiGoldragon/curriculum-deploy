use std::process::ExitCode;

use curriculum_deploy::CommandLine;

fn main() -> ExitCode {
    match CommandLine::from_arguments(std::env::args().skip(1)).run() {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
