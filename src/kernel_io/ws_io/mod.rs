//! The ws_io module provides a websocket-based interface
//! for controlling the kernel.

pub mod ws_command_adapter;
pub mod ws_command_consumption_port;
pub mod ws_command_parser;
pub mod ws_message_consumption_port;
pub mod ws_server;

pub async fn run_ws_io(addr: &'static str) {
    /*
    First we're going to initialize the curator
    TODO: Run the server assembler
    */
}
