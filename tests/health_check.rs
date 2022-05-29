use reqwest::StatusCode;

#[tokio::test]
async fn health_check_should_return_ok() {
    init_email_newsletter();

    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8080/health")
        .send()
        .await
        .expect("Error executing http request");

    let response_status = response.status();
    let response_body = response.text().await.expect("Error getting response body");

    assert_eq!(response_status, StatusCode::OK);
    assert_eq!(response_body,"{ \"status\": \"UP\" }")
}

fn init_email_newsletter() {
    let email_newsletter =
        email_newsletter::run().expect("failed to build `Email Newsletter` server");

    let _ = tokio::spawn(email_newsletter);
}
