//! The ws command consumption port provides a port
//! to a module that processes a parsed ws command.

use crate::kernel_io::ws_io::ws_command_adapter::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_command_adapter::ws_response::WsResponse;

pub trait WsCommandConsumptionPort {
    /// Consumes a parsed command and returns the ws
    /// response to the command from the curator
    ///
    /// First return is the actual response, second return
    /// is whether to terminate the server following the completion
    /// of the response
    fn consume_ws_command(&self, ws_command: WsCommand) -> (WsResponse, bool);
}
