use serde::{Deserialize, Serialize};

pub mod manifest;


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandleCreateClusterMessage {
    pub name: String,
}