use jph_cli::posts::Post;
use mockito::Server;

#[tokio::test]
async fn test_get_all_returns_posts() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/posts")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"id":1,"title":"foo","body":"bar","userId":1}]"#)
        .create_async()
        .await;

    let client = reqwest::Client::new();
    let url = format!("{}/posts", server.url());

    let posts = Post::get_all(&client, &url).await.unwrap();

    assert_eq!(posts.len(), 1);
    let expected_post = Post::new(1, "foo".into(), "bar".into(), 1);
    assert_eq!(posts.get(0).unwrap(), &expected_post);
    mock.assert();
}

#[tokio::test]
async fn test_get_all_on_500_server_returns_error() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/posts").with_status(500).create_async().await;
    let client = reqwest::Client::new();
    let url = format!("{}/posts", server.url());
    let posts = Post::get_all(&client, &url).await;
    assert!(posts.is_err());
}

#[tokio::test]
async fn test_get_all_returns_empty_vec_on_error() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/posts").create_async().await;

    let client = reqwest::Client::new();
    let url = format!("{}/posts", server.url());
    let posts = Post::get_all(&client, &url).await;
    assert!(posts.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_as_json_returns_json_value() {
    let post = Post::new(1, "foo".into(), "bar".into(), 1);
    let json = post.as_json().unwrap();
    assert_eq!(json, serde_json::json!({"id":1,"title":"foo","body":"bar","userId":1}));
}

