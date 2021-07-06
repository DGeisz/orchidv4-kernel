/// This port provides an interface that consumes
/// the raw text coming straight from ws messages,
/// and potentially provide responses to be sent back
/// to all websocket clients.
pub trait WsMessageConsumptionPort {
    /// Consumes a raw ws message and optionally returns
    /// text meant to be sent back through all ws clients
    fn consume_ws_message(&self, message: String) -> Option<String>;
}
