use mockito::ServerGuard;
use reqwest::Client;
use rstest::fixture;

#[fixture]
pub fn client() -> Client {
    Client::new()
}

#[fixture]
pub async fn server() -> ServerGuard {
    mockito::Server::new_async().await
}
