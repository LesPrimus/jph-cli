use clap::Parser;

pub mod todos;
pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cli::*;

    let cli = Cli::parse();
    let client = reqwest::Client::new();

    // Display any error from handler
    if let Err(e) = match cli.command {
        Command::Todos { command } => todos::ToDo::handle_cli_command(command, &client).await,
    } {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
    Ok(())
}

