use crate::curator::curator_control::CuratorControl;
use crate::kernel_io::ws_io::ws_com_res::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_com_res::ws_response::WsResponse;
use crate::kernel_io::ws_io::ws_command_adapter::ws_command_consumer::WsCommandConsumer;
use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::page_serialization::PageSerialization;

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
            WsCommand::FullPage { page_id } => match self.curator.get_page(page_id) {
                None => (WsResponse::Error, false),
                Some(page) => (WsResponse::FullPage { page }, false),
            },
            WsCommand::FillDecSocket {
                page_id,
                socket_id,
                dec_name,
            } => match self
                .curator
                .fill_dec_socket(page_id.clone(), socket_id, dec_name)
            {
                None => (WsResponse::Error, false),
                Some(dec_socket_ser) => (
                    WsResponse::DecSocketUpdate {
                        page_id,
                        dec_socket_ser,
                    },
                    false,
                ),
            },
            WsCommand::AppendDecSocket { page_id } => {
                match self.curator.append_dec_socket(page_id.clone()) {
                    None => (WsResponse::Error, false),
                    Some(dec_socket_ser) => (
                        WsResponse::DecSocketAppend {
                            page_id,
                            dec_socket_ser,
                        },
                        false,
                    ),
                }
            }
            WsCommand::DeleteDecSocket { page_id, socket_id } => {
                match self
                    .curator
                    .delete_dec_socket(page_id.clone(), socket_id.clone())
                {
                    false => (WsResponse::Error, false),
                    true => (
                        WsResponse::DecSocketDelete {
                            page_id,
                            dec_socket_id: socket_id,
                        },
                        false,
                    ),
                }
            }
        }
    }
}
