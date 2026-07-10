use clap::{Parser, Subcommand};
use std::fs;
use ts3::core::execute::VM;
use ts3::core::parser::parse_program;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { file: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { file } => {
            let content = fs::read_to_string(&file)?;
            let instructions = parse_program(&content)?;
            let mut vm = VM::new(instructions);
            vm.run()?;
        }
    }
    Ok(())
}
