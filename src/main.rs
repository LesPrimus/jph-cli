use clap::Parser;

pub mod todos;
pub mod cli;

#[tokio::main]
async fn main() {
    use cli::*;

    let cli = Cli::parse();
    match cli.command {
        Command::Todos { command } => match command {
            TodoCommand::List => {
                println!("Listing todos");
            }
            TodoCommand::Get { id } => {
                println!("Get a todos  with an id: {}", id);
            }
        },
    }
}

