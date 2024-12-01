use std::sync::Arc;
use anyhow::Error;
use tracing::info;
use crate::application::ports::messaging_ports::MessagingPort;
use crate::domain::cluster::entities::HandleCreateClusterMessage;
use crate::domain::cluster::ports::{MinecraftClusterRepository, MinecraftClusterService};

#[derive(Debug, Clone)]
pub struct ClusterServiceImpl<C, M>
where
    C: MinecraftClusterRepository,
    M: MessagingPort
{
    cluster_repository: C,
    messaging: Arc<M>,
}

impl<C, M> ClusterServiceImpl<C, M>
where
    C: MinecraftClusterRepository,
    M: MessagingPort
{
    pub fn new(cluster_repository: C, messaging: Arc<M>) -> Self {
        ClusterServiceImpl {
            cluster_repository,
            messaging,
        }
    }
}

impl<C, M> MinecraftClusterService for ClusterServiceImpl<C, M>
where
    C: MinecraftClusterRepository,
    M: MessagingPort
{
    async fn create_cluster(&self, _name: String) -> Result<(), anyhow::Error> {
        info!("Creating cluster");
        Ok(())
    }

    async fn handle_create_cluster(&self, message: HandleCreateClusterMessage) -> Result<(), Error> {
        info!("Handling create cluster message: {:?}", message);
        Ok(())
    }
}