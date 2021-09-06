use mockall::*;
use serde::{Deserialize, Serialize};

/// This port provides an interface that consumes
/// the raw text coming straight from ws messages,
/// and potentially provide responses to be sent back
/// to all websocket clients.
#[automock]
pub trait WsMessageConsumer {
    /// Consumes a raw ws message and optionally returns
    /// text meant to be sent back through all ws clients
    fn consume_ws_message(&mut self, message: String) -> MessageConsumptionResponse;
}

pub fn mock_ws_message_consumer() -> MockWsMessageConsumer {
    MockWsMessageConsumer::new()
}

/// The response from a message consumer
#[derive(Clone, Deserialize, Serialize, Eq, PartialEq, Debug)]
pub enum MessageConsumptionResponse {
    /// Indicates we have a message to send back
    /// to all ws clients
    Message(String),
    /// Indicates that the server should terminate
    /// after sending out this final message
    Terminate(String),
    /// Indicates that the server should continue, but
    /// no message should be sent back to ws clients
    None,
}
