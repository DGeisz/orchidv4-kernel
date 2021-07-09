use crate::kernel_io::ws_io::ws_command_adapter::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_command_adapter::ws_response::WsResponse;
use crate::kernel_io::ws_io::ws_command_consumption_port::WsCommandConsumptionPort;
use crate::kernel_io::ws_io::ws_message_consumption_port::{
    MessageConsumptionResponse, WsMessageConsumptionPort,
};
use log::error;

#[cfg(test)]
mod tests;

/// The struct that attempts to parse
/// the raw text from websocket messages
/// into meaningful commands
pub struct WsCommandParser {
    command_consumer: Box<dyn WsCommandConsumptionPort>,
}

impl WsCommandParser {
    pub fn new(
        command_consumer: Box<dyn WsCommandConsumptionPort>,
    ) -> Box<dyn WsMessageConsumptionPort> {
        Box::new(WsCommandParser { command_consumer })
    }
}

impl WsMessageConsumptionPort for WsCommandParser {
    fn consume_ws_message(&mut self, message: String) -> MessageConsumptionResponse {
        /*
        Attempt to deserialize the message
        */
        let de_result: Result<WsCommand, _> = serde_json::from_str(&message);

        error!("This is result: {:?}", de_result);

        match de_result {
            Ok(ws_command) => {
                let (response, terminate) = self.command_consumer.consume_ws_command(ws_command);

                /*
                Check to see if the response is an error,
                and if so, return none early
                */
                if let WsResponse::Error = &response {
                    return MessageConsumptionResponse::None;
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
