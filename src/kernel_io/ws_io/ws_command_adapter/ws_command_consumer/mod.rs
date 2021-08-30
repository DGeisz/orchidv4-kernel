use crate::kernel_io::ws_io::ws_command_adapter::ws_command_consumer::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_command_adapter::ws_command_consumer::ws_response::WsResponse;
use mockall::*;

pub mod ws_commands;
pub mod ws_response;

/// The ws command consumption port provides a port
/// to a module that processes a parsed ws command.
#[automock]
pub trait WsCommandConsumer {
    /// Consumes a parsed command and returns the ws
    /// response to the command from the curator
    ///
    /// First return is the actual response, second return
    /// is whether to terminate the server following the completion
    /// of the response
    fn consume_ws_command(&mut self, ws_command: WsCommand) -> (WsResponse, bool);
}

pub fn mock_ws_command_consumer() -> MockWsCommandConsumer {
    MockWsCommandConsumer::new()
}
