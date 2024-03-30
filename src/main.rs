use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, Responder};
use std::net::TcpListener;

async fn greet() -> impl Responder {
    "Hello World!".to_string()
}

fn build_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/", web::get().to(greet)))
        .listen(listener)?
        .run();

    Ok(server)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind random port!");

    let server = build_server(listener).expect("Failed to bind to address");

    server.await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spawn_app() -> String {
        let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind random port!");
        let port = listener.local_addr().unwrap().port();
        let server = build_server(listener).expect("Failed to bind to address");
        let _ = tokio::spawn(server);

        format!("http://0.0.0.0:{}", port)
    }

    #[tokio::test]
    async fn test_app_spawns() {
        spawn_app();
    }

    #[tokio::test]
    async fn test_greet() {
        let address = spawn_app();
        let client = reqwest::Client::new();

        let response = client
            .get(format!("{}/", address))
            .send()
            .await
            .expect("Failed to send request");

        assert!(response.status().is_success());

        if let Ok(value) = response.text().await {
            assert_eq!("Hello World!", value.as_str());
        } else {
            panic!("Failed to parse fetch response content!");
        };
    }
}
