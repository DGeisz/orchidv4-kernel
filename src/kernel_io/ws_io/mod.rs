//! The ws_io module provides a websocket-based interface
//! for controlling the kernel.

use crate::curator::assemble_kernel_curator;
use crate::kernel_io::ws_io::ws_command_adapter::WsCommandAdapter;
use crate::kernel_io::ws_io::ws_command_parser::WsCommandParser;
use crate::kernel_io::ws_io::ws_message_consumption_port::WsMessageConsumptionPort;
use crate::kernel_io::ws_io::ws_server::run_server;

pub mod ws_command_adapter;
pub mod ws_command_consumption_port;
pub mod ws_command_parser;
pub mod ws_message_consumption_port;
pub mod ws_server;

/// Generate a message consumption port with all the other
/// pieces of the puzzle (parser, adapter, curator) properly
/// assembles and ready to be run
fn generate_consumption_port() -> Box<dyn WsMessageConsumptionPort> {
    WsCommandParser::new(WsCommandAdapter::new(assemble_kernel_curator()))
}

/// Runs a ws io interface with all the pieces properly
/// assembled to run the full kernel
pub async fn run_ws_io(addr: &'static str) {
    /*
    Basically run the ws server with all the proper parts of
    ws_io put together
    */
    run_server(addr, generate_consumption_port).await;
}
