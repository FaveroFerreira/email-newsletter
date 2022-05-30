use std::net::TcpListener;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");

    let port = listener.local_addr().unwrap().port();

    let server = email_newsletter::startup::run(listener)
        .expect("failed to build `Email Newsletter` server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
