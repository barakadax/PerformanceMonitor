use clap::Parser;
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Prints each given argument and its position.",
    long_about = None,
    trailing_var_arg = true )]
pub struct Args {
    #[arg(num_args = 0..)]
    pub positional_args: Vec<String>,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }

    pub fn get_concat_args(&self) -> String {
        self.positional_args.join(" ")
    }
}