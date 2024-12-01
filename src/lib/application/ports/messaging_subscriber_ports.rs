use std::{fmt::Debug, future::Future};

use serde::de::DeserializeOwned;

pub trait MessagingSubscriberPort: Clone + Send + Sync + 'static {
    fn subscribe<F, T, Fut>(
        &self,
        topic: &str,
        handler: F,
    ) -> impl Future<Output = anyhow::Result<()>> + Send
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = anyhow::Result<()>> + Send + 'static,
        T: DeserializeOwned + Send + Sync + Debug + 'static;
}
