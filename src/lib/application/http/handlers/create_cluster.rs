use std::sync::Arc;

use axum::{http::StatusCode, Extension};

use crate::domain::cluster::ports::MinecraftClusterService;

use super::{ApiError, ApiSuccess};

pub async fn create_cluster<C: MinecraftClusterService>(
    Extension(cluster_service): Extension<Arc<C>>,
) -> Result<ApiSuccess<String>, ApiError> {
    cluster_service
        .create_cluster("Hypolia".to_string())
        .await?;

    Ok(ApiSuccess::new(
        StatusCode::CREATED,
        "Cluster created".to_string(),
    ))
}
