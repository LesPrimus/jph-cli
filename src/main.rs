pub mod todos;


#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let todos = todos::ToDo::get_all(&client).await.unwrap();
    dbg!(todos);
    let todo_1 = todos::ToDo::get_by_id(&client, 1).await.unwrap();
    dbg!(todo_1);
}

