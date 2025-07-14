use std::fmt::format;
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
    let url = spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", url))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
