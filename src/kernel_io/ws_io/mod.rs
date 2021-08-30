//! The ws_io module provides a websocket-based interface
//! for controlling the kernel.

pub mod ws_command_adapter;
pub mod ws_command_parser;
pub mod ws_server;

/// Runs a ws io interface with all the pieces properly
/// assembled to run the full kernel
pub async fn run_ws_io(_addr: &'static str) {
    /*
    Basically run the ws server with all the proper parts of
    ws_io put together

    TODO: Create a consumption port generator to run the server
    */
    // run_server(addr, generate_consumption_port).await;
}
