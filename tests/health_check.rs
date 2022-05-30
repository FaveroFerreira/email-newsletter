use reqwest::StatusCode;

mod utils;

#[tokio::test]
async fn health_check_should_return_ok() {
    let address = utils::spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", address))
        .send()
        .await
        .expect("Error executing http request");

    let response_status = response.status();
    let response_body = response.text().await.expect("Error getting response body");

    assert_eq!(StatusCode::OK, response_status);
    assert_eq!(r#"{ "status": "UP" }"#, response_body)
}
