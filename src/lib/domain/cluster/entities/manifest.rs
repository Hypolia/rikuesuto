use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "shulkermc.io",
    version = "v1alpha1",
    kind = "MinecraftCluster",
    namespaced
)]
pub struct MinecraftClusterSpec {
    pub name: String,
}
