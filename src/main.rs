use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
enum ToDoError {
    #[error(transparent)]
    Network(#[from] reqwest::Error),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ToDo {
    completed: bool,
    id: i32,
    title: String,
    user_id: i32,
}

impl ToDo {
    const TARGET: &str = "https://jsonplaceholder.typicode.com/todos";

    async fn get_all(client: &Client) -> Result<Vec<ToDo>, ToDoError> {
        Ok(client.get(Self::TARGET).send().await?.json().await?)
    }

    async fn get_by_id(client: &Client, id: i32) -> Result<ToDo, ToDoError> {
        let target = format!("{}/{}", Self::TARGET, id);
        Ok(client.get(target).send().await?.json().await?)
    }
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let todos = ToDo::get_all(&client).await.unwrap();
    dbg!(todos);
    let todo_1 = ToDo::get_by_id(&client, 1).await.unwrap();
    dbg!(todo_1);
}

