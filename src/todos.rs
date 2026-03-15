use crate::cli::TodoCommand;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum ToDoError {
    #[error(transparent)]
    Network(#[from] reqwest::Error),
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
        Ok(client.get(target).send().await?.json().await?)
    }

    pub async fn handle_cli_command(command: &TodoCommand, client: &Client) {
        todo!()
    }
}