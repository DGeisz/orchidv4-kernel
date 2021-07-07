use serde::{Deserialize, Serialize};

/// This is a serializable output that can be sent
/// back to the client
#[derive(Eq, PartialEq)]
pub enum WsResponse {
    NewPage { page_id: String },
}
