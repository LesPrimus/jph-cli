use crate::cli::{CommandHandler, PostCommand};
use crate::errors::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum PostsError {
    #[error(transparent)]
    Network(#[from] reqwest::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    user_id: i32,
}

impl Post {
    const TARGET: &str = "https://jsonplaceholder.typicode.com/posts";

    pub async fn get_all(client: &reqwest::Client) -> Result<Vec<Post>, PostsError> {
        Ok(client.get(Self::TARGET).send().await?.json().await?)
    }

    pub fn as_json(&self) -> Result<serde_json::Value, PostsError> {
        Ok(serde_json::to_value(self)?)
    }
}

impl CommandHandler for Post {
    type Command = PostCommand;

    async fn handle_cli_command(
        command: Self::Command,
        client: &reqwest::Client,
    ) -> Result<(), AppError> {
        match command {
            PostCommand::List => {
                for post in Self::get_all(client).await?.iter() {
                    println!("{}", post.as_json()?);
                }
            }
        }
        Ok(())
    }
}
