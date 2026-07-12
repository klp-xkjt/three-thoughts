use anyhow::Result;
use clap::{Parser, Subcommand};

use std::fs;
use threethoughts::core::execute::VM;
use threethoughts::core::parser::parse_program;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        file: String,

        #[arg(long, default_value = "65536")]
        mem: usize,

        #[arg(long, default_value = "false")]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { file, mem, debug } => {
            let content = fs::read_to_string(&file)?;
            let instructions = parse_program(&content)?;
            let mut vm = VM::new(instructions, mem, debug);
            vm.run()?;
        }
    }
    Ok(())
}
