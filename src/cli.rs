use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Todos {
        #[command(subcommand)]
        command: TodoCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum TodoCommand {
    List,
    Get { id: i32 },
}