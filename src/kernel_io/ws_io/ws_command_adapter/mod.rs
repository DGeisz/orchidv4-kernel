use crate::curator::curator_control::CuratorControl;
use crate::kernel_io::ws_io::ws_com_res::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_com_res::ws_response::WsResponse;
use crate::kernel_io::ws_io::ws_command_adapter::ws_command_consumer::WsCommandConsumer;

pub mod ws_command_consumer;

#[cfg(test)]
mod tests;

pub struct WsCommandAdapter {
    curator: Box<dyn CuratorControl>,
}

impl WsCommandAdapter {
    pub fn new(curator: Box<dyn CuratorControl>) -> Box<dyn WsCommandConsumer> {
        Box::new(WsCommandAdapter { curator })
    }
}

impl WsCommandConsumer for WsCommandAdapter {
    fn consume_ws_command(&mut self, ws_command: WsCommand) -> (WsResponse, bool) {
        match ws_command {
            WsCommand::NewPage { target_client } => (
                WsResponse::NewPage {
                    target_client,
                    new_page: self.curator.new_page(),
                },
                false,
            ),
        }
    }
}
