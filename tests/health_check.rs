use zero2prod;

fn spawn_app() {
    let server = zero2prod::run().expect("Unable to start server");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn test_health_check() {
    // Arrange
    spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://localhost:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
