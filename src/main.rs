mod engine;
mod rules;
mod shell;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fuck", about = "thefuck-rs: corrects your last command")]
struct Cli {
    #[command(subcommand)]
    command: Option<Cmd>,
    #[arg(long)]
    alias: bool,
}

#[derive(Subcommand)]
enum Cmd {
    Fix,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.alias {
        shell::print_alias();
        return Ok(());
    }

    engine::run()
}
