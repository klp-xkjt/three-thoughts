use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { file } => {
            println!("File: {}", file)
        }
    }
}
