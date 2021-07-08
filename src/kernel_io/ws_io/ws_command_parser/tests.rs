use crate::kernel_io::ws_io::ws_command_adapter::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_command_adapter::ws_response::WsResponse;
use crate::kernel_io::ws_io::ws_command_consumption_port::mock_ws_command_consumption_port;
use crate::kernel_io::ws_io::ws_command_parser::WsCommandParser;
use crate::kernel_io::ws_io::ws_message_consumption_port::MessageConsumptionResponse;
use mockall::predicate::eq;

/*
As a quick note about tests in this module,
these tests aren't to ensure that every single command works
properly, but rather that the whole serialization/deserialization
flow works properly.  Thus only a small subset of commands are covered here
*/

#[test]
/// Tests the implementation of new page commands.
/// This is
fn test_new_page() {
    /*
    Create a mocked ws command consumption port
    */
    let mut mocked_port = Box::new(mock_ws_command_consumption_port());

    let page_id = String::from("page_id");
    let pid_clone = page_id.clone();

    mocked_port
        .expect_consume_ws_command()
        .with(eq(WsCommand::NewPage))
        .times(1)
        .return_const((WsResponse::NewPage { page_id }, false));

    /*
    Whip up a command parser
    */
    let mut command_parser = WsCommandParser::new(mocked_port);

    let command = serde_json::to_string(&WsCommand::NewPage).unwrap();

    /*
    Intake a new page command
    */
    let response = command_parser.consume_ws_message(command);

    if let MessageConsumptionResponse::Message(text) = response {
        let expected_message =
            serde_json::to_string(&WsResponse::NewPage { page_id: pid_clone }).unwrap();

        assert_eq!(expected_message, text);
    } else {
        panic!("Should have received a message command response");
    }
}
