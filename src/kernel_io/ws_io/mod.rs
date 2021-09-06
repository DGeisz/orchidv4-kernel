//! The ws_io module provides a websocket-based interface
//! for controlling the kernel.

use crate::curator::Curator;
use crate::kernel_io::ws_io::ws_command_adapter::WsCommandAdapter;
use crate::kernel_io::ws_io::ws_command_parser::ws_message_consumer::WsMessageConsumer;
use crate::kernel_io::ws_io::ws_command_parser::WsCommandParser;
use crate::kernel_io::ws_io::ws_server::run_server;
use crate::page::page_generator::PageGenerator;

pub mod ws_com_res;
pub mod ws_command_adapter;
pub mod ws_command_parser;
pub mod ws_server;

fn assemble_kernel_with_ws_msg_consumer() -> Box<dyn WsMessageConsumer> {
    let page_generator = PageGenerator::new();
    let curator = Curator::new(page_generator);
    let ws_command_adapter = WsCommandAdapter::new(curator);

    WsCommandParser::new(ws_command_adapter)
}

/// Runs a ws io interface with all the pieces properly
/// assembled to run the full kernel
pub async fn run_ws_io(addr: &'static str) {
    /*
    Basically run the ws server with all the proper parts of
    ws_io put together
    */
    run_server(addr, assemble_kernel_with_ws_msg_consumer).await;
}
