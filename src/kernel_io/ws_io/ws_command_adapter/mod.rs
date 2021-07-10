use crate::curator::curator_control_port::CuratorControlPort;
use crate::kernel_io::ws_io::ws_command_adapter::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_command_adapter::ws_response::WsResponse;
use crate::kernel_io::ws_io::ws_command_consumption_port::WsCommandConsumptionPort;

pub mod ws_commands;
pub mod ws_response;

#[cfg(test)]
mod tests;

pub struct WsCommandAdapter {
    curator_controller: Box<dyn CuratorControlPort>,
}

impl WsCommandAdapter {
    pub fn new(
        curator_controller: Box<dyn CuratorControlPort>,
    ) -> Box<dyn WsCommandConsumptionPort> {
        Box::new(WsCommandAdapter { curator_controller })
    }
}

impl WsCommandConsumptionPort for WsCommandAdapter {
    fn consume_ws_command(&mut self, ws_command: WsCommand) -> (WsResponse, bool) {
        match ws_command {
            WsCommand::NewPage => {
                /*
                This is pretty simple.  Basically just instruct the curator
                controller controller to create a new page, and grab its id
                */
                let page_id = self.curator_controller.new_page();

                /*
                Return a new page response, with the second arg indicating
                we most certainly don't want to terminate the server
                */
                (WsResponse::NewPage { page_id }, false)
            }
            WsCommand::FullPage(page_cmd) => {
                /*
                Simply call full page command, and you should either
                get back a full page serialization, or an error
                */
                match self.curator_controller.full_page(page_cmd.page_id) {
                    Ok(page) => (WsResponse::FullPage { page }, false),
                    Err(_) => (WsResponse::Error, false),
                }
            }
            WsCommand::CreateFeature {
                page_id,
                socket_id,
                feature,
            } => {
                match self
                    .curator_controller
                    .create_feature(page_id, socket_id, feature)
                {
                    Ok(page) => (WsResponse::FullPage { page }, false),
                    Err(_) => (WsResponse::Error, false),
                }
            }
        }
    }
}
