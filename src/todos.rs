use crate::cli::TodoCommand;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum ToDoError {
    #[error(transparent)]
    Network(#[from] reqwest::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("todo with id {0} not found")]
    NotFound(i32),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToDo {
    completed: bool,
    id: i32,
    title: String,
    user_id: i32,
}

impl ToDo {
    const TARGET: &str = "https://jsonplaceholder.typicode.com/todos";

    pub async fn get_all(client: &Client) -> Result<Vec<ToDo>, ToDoError> {
        Ok(client.get(Self::TARGET).send().await?.json().await?)
    }

    pub async fn get_by_id(client: &Client, id: i32) -> Result<ToDo, ToDoError> {
        let target = format!("{}/{}", Self::TARGET, id);
        let value: serde_json::Value = client.get(target).send().await?.json().await?;
        // Handle empty json object
        if value.as_object().map(|o| o.is_empty()).unwrap_or(false) {
            return Err(ToDoError::NotFound(id));
        }
        Ok(serde_json::from_value(value)?)
    }

    pub async fn handle_cli_command(command: TodoCommand, client: &Client) -> Result<(), ToDoError> {
        match command {
            TodoCommand::List => {
                for todo in Self::get_all(client).await?.iter() {
                    println!("{}", serde_json::to_string(todo)?);
                }
            }
            TodoCommand::Get { id } => {
                let todo = Self::get_by_id(client, id).await?;
                println!("{}", serde_json::to_string(&todo)?)
            }
        }
        Ok(())
    }
}