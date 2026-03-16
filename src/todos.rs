use crate::cli::{CommandHandler, TodoCommand};
use crate::errors::AppError;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum ToDoError {
    #[error(transparent)]
    Network(#[from] reqwest::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("todo with id {0} not found")]
    NotFound(String),
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
    // const TARGET: &str = "https://jsonplaceholder.typicode.com/todos";

    pub async fn get_all(client: &Client, url: &str) -> Result<Vec<ToDo>, ToDoError> {
        Ok(client.get(url).send().await?.json().await?)
    }

    pub async fn get_by_id(client: &Client, url: &str) -> Result<ToDo, ToDoError> {
        let value: serde_json::Value = client.get(url).send().await?.json().await?;
        // Handle empty json object
        if value.as_object().map(|o| o.is_empty()).unwrap_or(false) {
            return Err(ToDoError::NotFound(url.into()));
        }
        Ok(serde_json::from_value(value)?)
    }

    pub async fn create(
        title: String,
        user_id: i32,
        completed: bool,
        client: &Client,
        url: &str,
    ) -> Result<StatusCode, ToDoError> {
        let response = client
            .post(url)
            .json(&serde_json::json!({
                "title": title,
                "userId": user_id,
                "completed": completed,
            }))
            .send()
            .await?;
        Ok(response.status())
    }

    pub fn as_json(&self) -> Result<serde_json::Value, ToDoError> {
        Ok(serde_json::to_value(self)?)
    }
}

impl CommandHandler for ToDo {
    type Command = TodoCommand;
    const TARGET: &str = "https://jsonplaceholder.typicode.com/todos";

    async fn handle_cli_command(command: Self::Command, client: &Client) -> Result<(), AppError> {
        match command {
            TodoCommand::List => {
                for todo in Self::get_all(client, Self::TARGET).await?.iter() {
                    println!("{}", todo.as_json()?);
                }
            }
            TodoCommand::Get { id } => {
                let url = format!("{}/{}", Self::TARGET, id);
                let todo = Self::get_by_id(client, &url).await?;
                println!("{}", todo.as_json()?);
            }
            TodoCommand::Create {
                title,
                user_id,
                completed,
            } => {
                let response_status = Self::create(title, user_id, completed, client, Self::TARGET).await?;
                println!("{}", response_status);
            }
        }
        Ok(())
    }
}
