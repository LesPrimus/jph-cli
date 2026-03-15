use clap::Parser;

pub mod todos;
pub mod cli;

#[tokio::main]
async fn main() {
    use cli::*;

    let cli = Cli::parse();
    let client = reqwest::Client::new();
    match cli.command {
        Command::Todos { command } => match command {
            TodoCommand::List => {
                let todos = todos::ToDo::get_all(&client).await.unwrap_or_else(|e| {
                    eprintln!("{:?}", e);
                    std::process::exit(1);
                });
                todos.iter().for_each(|todo| {
                    let json = serde_json::to_string(&todo).unwrap_or_else(|e| {
                        eprintln!("{:?}", e);
                        std::process::exit(1);
                    });
                    println!("{}", json);
                })
            }
            TodoCommand::Get { id } => {
                let todo = todos::ToDo::get_by_id(&client, id).await.unwrap_or_else(|e| {
                    eprintln!("{:?}", e);
                    std::process::exit(1);
                });
                let json = serde_json::to_string(&todo).unwrap_or_else(|e| {
                    eprintln!("{:?}", e);
                    std::process::exit(1);
                });
                println!("{}", json);
            }
        },
    }
}

