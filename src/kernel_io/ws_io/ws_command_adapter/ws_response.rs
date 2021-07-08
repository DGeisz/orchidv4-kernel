use crate::page::page_serialization::PageSerialization;
use serde::{Deserialize, Serialize};

/// This is a serializable output that can be sent
/// back to the client
#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum WsResponse {
    NewPage { page_id: String },
    FullPage { page: PageSerialization },
    Error,
}
