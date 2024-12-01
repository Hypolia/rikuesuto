use std::future::Future;
use crate::domain::cluster::entities::HandleCreateClusterMessage;
use super::entities::manifest::MinecraftCluster;

pub trait MinecraftClusterRepository: Clone + Send + Sync + 'static {
    fn create(&self, cluster: MinecraftCluster) -> impl Future<Output = Result<(), anyhow::Error>>;
}

pub trait MinecraftClusterService: Clone + Send + Sync + 'static {
    fn create_cluster(&self, name: String) -> impl Future<Output = Result<(), anyhow::Error>>;
    fn handle_create_cluster(&self, message: HandleCreateClusterMessage) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}
