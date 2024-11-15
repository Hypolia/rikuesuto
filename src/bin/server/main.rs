use rikuesuto::application::http::{HttpServer, HttpServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config = HttpServerConfig::new("3333".to_string());
    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await
}
