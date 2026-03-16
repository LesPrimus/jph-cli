use clap::{Parser, Subcommand};
use reqwest::Client;
use std::future::Future;

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
    Get {
        id: i32,
    },
    Create {
        title: String,
        user_id: i32,
        #[arg(long)]
        completed: bool,
    },
}

pub trait CommandHandler {
    type Command;
    type Error;
    fn handle_cli_command(
        command: Self::Command,
        client: &Client,
    ) -> impl Future<Output = Result<(), Self::Error>>;
}
