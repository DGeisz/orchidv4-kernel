use crate::curator::curator_control_port::MockCuratorControlPort;
use crate::kernel_io::ws_io::ws_command_adapter::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_command_adapter::ws_response::WsResponse;
use crate::kernel_io::ws_io::ws_command_adapter::WsCommandAdapter;
use crate::kernel_io::ws_io::ws_command_consumption_port::WsCommandConsumptionPort;

/// Tests the new page f11y in the ws command adaptor
#[test]
fn test_new_page() {
    /*
    First let's create a mock of the curator control port
    */
    let mut mock_port = Box::new(MockCuratorControlPort::new());

    /*
    Create the page id that we expect it to return
    */
    let page_id = String::from("page_id");
    let pid_clone = page_id.clone();

    mock_port.expect_new_page().times(1).return_const(page_id);

    /*
    Create the command adapter
    */
    let mut command_adapter = WsCommandAdapter::new(mock_port);

    /*
    Input a NewPage command
    */
    let (ws_res, terminate) = command_adapter.consume_ws_command(WsCommand::NewPage);

    /*
    We should get a new page response back
    */
    assert_eq!(ws_res, WsResponse::NewPage { page_id: pid_clone });

    /*
    Terminate should be false
    */
    assert_eq!(terminate, false);
}
