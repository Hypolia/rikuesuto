use std::sync::Arc;
use anyhow::Result;
use crate::application::ports::messaging_subscriber_ports::MessagingSubscriberPort;
use crate::domain::cluster::entities::HandleCreateClusterMessage;
use crate::domain::cluster::ports::MinecraftClusterService;

pub async fn start_subscriptions<M, C>(
    cluster_service: Arc<C>,
    messaging: Arc<M>
) -> Result<()>
where
    M: MessagingSubscriberPort,
    C: MinecraftClusterService
{
    let messaging = Arc::clone(&messaging);
    let cluster_service = Arc::clone(&cluster_service);

    tokio::spawn(async move {
       loop {
           let cluster_service = Arc::clone(&cluster_service);
           if let Err(e) = messaging.subscribe("create_cluster", move |message: HandleCreateClusterMessage| {
               let cluster_service = Arc::clone(&cluster_service);
               async move {
                   cluster_service.handle_create_cluster(message).await?;
                   Ok(())
               }
           }).await {
               tracing::error!("Error subscribing to topic: {:?}", e);

           }
       }
    });

    Ok(())
}