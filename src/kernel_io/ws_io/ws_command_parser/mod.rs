use crate::kernel_io::ws_io::ws_com_res::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_com_res::ws_response::{WsError, WsResponse};
use crate::kernel_io::ws_io::ws_command_adapter::ws_command_consumer::WsCommandConsumer;
use crate::kernel_io::ws_io::ws_command_parser::ws_message_consumer::{
    MessageConsumptionResponse, WsMessageConsumer,
};
use log::trace;

pub mod ws_message_consumer;

#[cfg(test)]
mod tests;

/// The struct that attempts to parse
/// the raw text from websocket messages
/// into meaningful commands
pub struct WsCommandParser {
    command_consumer: Box<dyn WsCommandConsumer>,
}

impl WsCommandParser {
    pub fn new(command_consumer: Box<dyn WsCommandConsumer>) -> Box<dyn WsMessageConsumer> {
        Box::new(WsCommandParser { command_consumer })
    }
}

impl WsMessageConsumer for WsCommandParser {
    fn consume_ws_message(&mut self, message: String) -> MessageConsumptionResponse {
        /*
        Attempt to deserialize the message
        */
        let de_result: Result<WsCommand, _> = serde_json::from_str(&message);

        trace!("This is result: {:?}", de_result);

        match de_result {
            Ok(ws_command) => {
                let (response, terminate) = self.command_consumer.consume_ws_command(ws_command);

                /*
                Check if the response is an error with a no-op,
                and if so return None early
                */
                if let WsResponse::Error(err) = &response {
                    if let WsError::NoOp = err {
                        return MessageConsumptionResponse::None;
                    }
                }

                let ser_res = serde_json::to_string(&response).unwrap();

                /*
                Send a termination message back to the server if terminate is flagged
                */
                if terminate {
                    MessageConsumptionResponse::Terminate(ser_res)
                } else {
                    MessageConsumptionResponse::Message(ser_res)
                }
            }
            Err(_) => MessageConsumptionResponse::None,
        }
    }
}
