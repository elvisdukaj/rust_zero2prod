use std::net::TcpListener;
use zero2prod;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Unable to bind random port");
    let url = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let server = zero2prod::run(listener).expect("Unable to start server");
    let _ = tokio::spawn(server);
    url
}

#[tokio::test]
async fn test_health_check() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=my%20user&email=mu%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("email=mu%40gmail.com", "Missing name"),
        ("name=my%20user", "missing email"),
        ("", "Missing name and email"),
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            response.status(),
            reqwest::StatusCode::BAD_REQUEST,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
