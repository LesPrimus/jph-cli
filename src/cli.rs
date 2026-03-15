use std::future::Future;
use clap::{Parser, Subcommand};
use reqwest::Client;

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

pub trait CommandHandler {
    type Command;
    type Error;
    fn handle_cli_command(command: Self::Command, client: &Client) -> impl Future<Output=Result<(), Self::Error>>;
}
