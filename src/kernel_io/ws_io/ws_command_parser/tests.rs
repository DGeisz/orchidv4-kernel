use crate::kernel_io::ws_io::ws_com_res::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_com_res::ws_response::WsResponse;
use crate::kernel_io::ws_io::ws_command_adapter::ws_command_consumer::mock_ws_command_consumer;
use crate::kernel_io::ws_io::ws_command_parser::ws_message_consumer::MessageConsumptionResponse;
use crate::kernel_io::ws_io::ws_command_parser::WsCommandParser;
use crate::page::page_serialization::PageSerialization;
use mockall::predicate::eq;

#[test]
fn test_message_consumption() {
    /*
    Create the mocked command consumer
    */
    let mut mock_command_consumer = mock_ws_command_consumer();

    /*
    For the first message, we'll send in something that definitely isn't a command
    */
    let msg1 = "I'm not a command".to_string();

    /*
    For the second message, send in a new page command
    */
    let target_client = "client-id".to_string();
    let raw_msg2 = WsCommand::NewPage {
        target_client: target_client.clone(),
    };

    let msg2 = serde_json::to_string(&raw_msg2).unwrap();

    /*
    The response will be a new page
    */
    let page_id = "Here i am".to_string();

    let raw_res2 = WsResponse::NewPage {
        target_client,
        new_page: PageSerialization::new(page_id, vec![]),
    };

    let res2 = serde_json::to_string(&raw_res2).unwrap();

    /*
    Add true to the termination request so that we can
    ensure it is passed through
    */
    mock_command_consumer
        .expect_consume_ws_command()
        .times(1)
        .with(eq(raw_msg2.clone()))
        .return_const((raw_res2.clone(), true));

    mock_command_consumer
        .expect_consume_ws_command()
        .times(1)
        .with(eq(raw_msg2))
        .return_const((raw_res2.clone(), false));

    let mut box_mock = Box::new(mock_command_consumer);

    /*
    Create the ws command parser to test
    */
    let mut command_parser = WsCommandParser::new(box_mock);

    assert_eq!(
        command_parser.consume_ws_message(msg1),
        MessageConsumptionResponse::None
    );

    assert_eq!(
        command_parser.consume_ws_message(msg2.clone()),
        MessageConsumptionResponse::Terminate(res2.clone())
    );

    assert_eq!(
        command_parser.consume_ws_message(msg2),
        MessageConsumptionResponse::Message(res2)
    );
}
