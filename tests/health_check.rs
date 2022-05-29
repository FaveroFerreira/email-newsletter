use reqwest::StatusCode;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_should_return_ok() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", address))
        .send()
        .await
        .expect("Error executing http request");

    let response_status = response.status();
    let response_body = response.text().await.expect("Error getting response body");

    assert_eq!(response_status, StatusCode::OK);
    assert_eq!(response_body, "{ \"status\": \"UP\" }")
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");

    let port = listener.local_addr().unwrap().port();

    let server =
        email_newsletter::run(listener).expect("failed to build `Email Newsletter` server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
