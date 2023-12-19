#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let address = format!("{}/health_check", &address);
    println!("{}", address);
    let response = client
        .get(address)
        .send()
        .await
        .expect("Failed to execute our request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind port");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
