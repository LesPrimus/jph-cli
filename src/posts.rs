use crate::cli::{CommandHandler, PostCommand};
use crate::errors::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum PostsError {
    #[error(transparent)]
    Network(#[from] reqwest::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("post with id {0} not found")]
    NotFound(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    user_id: i32,
}

impl Post {
    pub fn new(id: i32, title: String, body: String, user_id: i32) -> Self {
        Self {
            id,
            title,
            body,
            user_id,
        }
    }

    pub async fn get_all(client: &reqwest::Client, url: &str) -> Result<Vec<Post>, PostsError> {
        Ok(client.get(url).send().await?.json().await?)
    }

    pub async fn get_by_id(client: &reqwest::Client, url: &str) -> Result<Post, PostsError> {
        let json_value: serde_json::Value = client.get(url).send().await?.json().await?;
        if json_value.is_null() {
            return Err(PostsError::NotFound(url.into()));
        }
        Ok(serde_json::from_value(json_value)?)
    }

    pub fn as_json(&self) -> Result<serde_json::Value, PostsError> {
        Ok(serde_json::to_value(self)?)
    }
}

impl CommandHandler for Post {
    type Command = PostCommand;
    const TARGET: &str = "https://jsonplaceholder.typicode.com/posts";

    async fn handle_cli_command(
        command: Self::Command,
        client: &reqwest::Client,
    ) -> Result<(), AppError> {
        match command {
            PostCommand::List => {
                for post in Self::get_all(client, Self::TARGET).await?.iter() {
                    println!("{}", post.as_json()?);
                }
            }
            PostCommand::Get { id } => {
                let url = format!("{}/{}", Self::TARGET, id);
                let post = Self::get_by_id(client, &url).await?;
                println!("{}", post.as_json()?);
            }
        }
        Ok(())
    }
}
