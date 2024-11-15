use std::future::Future;

use super::entities::manifest::MinecraftCluster;

pub trait MinecraftClusterRepository {
    fn create(&self, cluster: MinecraftCluster) -> impl Future<Output = Result<(), anyhow::Error>>;
}

pub trait MinecraftClusterService {
    fn create_cluster(&self, name: String) -> impl Future<Output = Result<(), anyhow::Error>>;
}
