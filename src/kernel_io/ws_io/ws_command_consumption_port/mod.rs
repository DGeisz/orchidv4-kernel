//! The ws command consumption port provides a port
//! to a module that presumably consumes and processes
//! the text of the command.

pub trait WsCommandConsumptionPort {
    /// Consumes a raw command from a websocket and potentially
    /// returns a string of text meant to be transmitted to all
    /// open websocket connections
    fn consume_ws_command(ws_command: String) -> Option<String>;
}
