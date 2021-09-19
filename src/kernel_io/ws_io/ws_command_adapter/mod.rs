use crate::curator::curator_control::CuratorControl;
use crate::kernel_io::ws_io::ws_com_res::ws_commands::{
    DecSocketCommand, TermDefSocketCommand, WsCommand,
};
use crate::kernel_io::ws_io::ws_com_res::ws_response::{
    DecSocketRes, TermDefSocketRes, WsError, WsResponse,
};
use crate::kernel_io::ws_io::ws_command_adapter::ws_command_consumer::WsCommandConsumer;
use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::lexicon::term_def::term_def_serialization::TermDefSocketSer;
use crate::page::page_serialization::PageSerialization;

pub mod ws_command_consumer;

const NO_OP: (WsResponse, bool) = (WsResponse::Error(WsError::NoOp), false);

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
                None => NO_OP,
                Some(page) => (WsResponse::FullPage { page }, false),
            },
            WsCommand::DecSocket { page_id, cmd } => match cmd {
                DecSocketCommand::Fill {
                    socket_id,
                    dec_name,
                } => match self.curator.fill_dec_socket(&page_id, socket_id, dec_name) {
                    None => NO_OP,
                    Some(dec_socket_ser) => (
                        WsResponse::DecSocket {
                            page_id,
                            res: DecSocketRes::Update { dec_socket_ser },
                        },
                        false,
                    ),
                },
                DecSocketCommand::Append => match self.curator.append_dec_socket(&page_id) {
                    None => NO_OP,
                    Some(dec_socket_ser) => (
                        WsResponse::DecSocket {
                            page_id,
                            res: DecSocketRes::Append { dec_socket_ser },
                        },
                        false,
                    ),
                },
                DecSocketCommand::Delete { socket_id } => {
                    match self.curator.delete_dec_socket(&page_id, socket_id.clone()) {
                        false => NO_OP,
                        true => (
                            WsResponse::DecSocket {
                                page_id,
                                res: DecSocketRes::Delete {
                                    dec_socket_id: socket_id,
                                },
                            },
                            false,
                        ),
                    }
                }
                DecSocketCommand::DeleteContents { socket_id } => {
                    match self.curator.delete_dec_socket_contents(&page_id, socket_id) {
                        None => NO_OP,
                        Some(dec_socket_ser) => (
                            WsResponse::DecSocket {
                                page_id,
                                res: DecSocketRes::Update { dec_socket_ser },
                            },
                            false,
                        ),
                    }
                }
                DecSocketCommand::Insert {
                    rel_socket_id,
                    before_rel,
                } => {
                    match self
                        .curator
                        .insert_dec_socket(&page_id, &rel_socket_id, before_rel)
                    {
                        None => NO_OP,
                        Some(dec_socket_ser) => (
                            WsResponse::DecSocket {
                                page_id,
                                res: DecSocketRes::Insert {
                                    rel_socket_id,
                                    before_rel,
                                    dec_socket_ser,
                                },
                            },
                            false,
                        ),
                    }
                }
            },
            WsCommand::TermDefSocket { page_id, cmd } => match cmd {
                TermDefSocketCommand::Fill { tds_id, term_seq } => {
                    match self
                        .curator
                        .fill_term_def_socket(&page_id, &tds_id, term_seq)
                    {
                        None => (
                            WsResponse::Error(WsError::InvalidTdsInput {
                                page_id,
                                socket_id: tds_id,
                            }),
                            false,
                        ),
                        Some(term_def_socket_ser) => (
                            WsResponse::TermDefSocket {
                                page_id,
                                res: TermDefSocketRes::Update {
                                    term_def_socket_ser,
                                },
                            },
                            false,
                        ),
                    }
                }
                TermDefSocketCommand::DeleteContents { tds_id } => {
                    match self.curator.delete_tds_contents(&page_id, &tds_id) {
                        None => NO_OP,
                        Some(term_def_socket_ser) => (
                            WsResponse::TermDefSocket {
                                page_id,
                                res: TermDefSocketRes::Update {
                                    term_def_socket_ser,
                                },
                            },
                            false,
                        ),
                    }
                }
            },
        }
    }
}
