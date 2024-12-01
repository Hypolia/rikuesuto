use clap::Parser;
use rikuesuto::application::http::{HttpServer, HttpServerConfig};
use rikuesuto::env::Env;
use rikuesuto::infrastructure::messaging::nats::Nats;
use std::sync::Arc;
use kube::Client;
use rikuesuto::application::messaging::start_subscriptions;
use rikuesuto::domain::cluster::services::ClusterServiceImpl;
use rikuesuto::infrastructure::repositories::kube_minecraft_cluster_repository::KubeMinecraftClusterRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let messaging = Arc::new(Nats::new(&env.nats_url).await?);


    let cluster_repository = KubeMinecraftClusterRepository::new(Client::try_default().await?);
    let cluster_service = Arc::new(ClusterServiceImpl::new(cluster_repository, Arc::clone(&messaging)));

    start_subscriptions(
        Arc::clone(&cluster_service),
        Arc::clone(&messaging),
    ).await?;

    let server_config = HttpServerConfig::new("3333".to_string());
    let http_server = HttpServer::new(server_config, Arc::clone(&cluster_service)).await?;

    http_server.run().await
}
