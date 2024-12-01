use kube::{api::PostParams, Api, Client};

use crate::domain::cluster::{
    entities::manifest::MinecraftCluster, ports::MinecraftClusterRepository,
};

#[derive(Clone)]
pub struct KubeMinecraftClusterRepository {
    client: Client,
}

impl KubeMinecraftClusterRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl MinecraftClusterRepository for KubeMinecraftClusterRepository {
    async fn create(&self, cluster: MinecraftCluster) -> Result<(), anyhow::Error> {
        let api: Api<MinecraftCluster> = Api::namespaced(self.client.clone(), "default");

        api.create(&PostParams::default(), &cluster).await?;

        Ok(())
    }
}
