use clap::Parser;

pub mod todos;
pub mod cli;

#[tokio::main]
async fn main() {
    use cli::*;

    let cli = Cli::parse();
    let client = reqwest::Client::new();
    match cli.command {
        Command::Todos { command } => todos::ToDo::handle_cli_command(&command, &client).await,
    }
}

