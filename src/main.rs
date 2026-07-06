mod app;
mod cli;
mod error;
mod exercises;
mod git;
mod output;
mod result;
mod safety;
mod state;
mod validators;

fn main() {
    if let Err(error) = cli::run(std::env::args().collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
