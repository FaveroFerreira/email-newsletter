use reqwest::StatusCode;

mod utils;

#[tokio::test]
async fn subscribe_returns_ok_for_valid_form_data() {
    let address = utils::spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/subscribe", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=Guilherme%20Ferreira&email=guilherme%40gmail.com")
        .send()
        .await
        .expect("Error executing http request");

    assert_eq!(StatusCode::OK, response.status())
}

#[tokio::test]
async fn subscribe_returns_400_when_form_data_is_missing() {
    let address = utils::spawn_app();

    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=Guilherme%20Ferreira", "missing the email"),
        ("email=guilherme%40gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Error executing http request");

        assert_eq!(
            StatusCode::BAD_REQUEST,
            response.status(),
            "The API did not failt with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
