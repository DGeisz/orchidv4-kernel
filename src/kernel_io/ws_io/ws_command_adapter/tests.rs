use crate::curator::curator_control_port::mock_curator_control_port;
use crate::kernel_io::ws_io::ws_command_adapter::ws_commands::{SimplePageCommand, WsCommand};
use crate::kernel_io::ws_io::ws_command_adapter::ws_response::WsResponse;
use crate::kernel_io::ws_io::ws_command_adapter::WsCommandAdapter;
use crate::kernel_io::ws_io::ws_command_consumption_port::WsCommandConsumptionPort;
use crate::page::feature_tree::feature::feature_serialization::FeatureSerialization;
use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use crate::page::page_error::PageError;
use crate::page::page_serialization::PageSerialization;
use mockall::predicate::eq;
use mockall::Sequence;

/// Tests the new page f11y in the ws command adaptor
#[test]
fn test_new_page() {
    /*
    First let's create a mock of the curator control port
    */
    let mut mock_port = Box::new(mock_curator_control_port());

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

/// Test full page f11y
#[test]
fn test_full_page() {
    /*
    Mock curator
    */
    let mut mock_curator = Box::new(mock_curator_control_port());

    let page_id = String::from("page_id");

    /*
    Create a mockall sequence to ensure commands get
    called in the correct order
    */
    let mut seq = Sequence::new();

    let pid_clone = page_id.clone();

    /*
    First, assume it returns an error
    */
    mock_curator
        .expect_full_page()
        .times(1)
        .with(eq(page_id.clone()))
        .in_sequence(&mut seq)
        .return_once(move |_| Err(PageError::PageNotFound(pid_clone)));

    let feature_socket = FeatureSocketSerialization::new(FeatureSerialization::None);
    let page: PageSerialization = PageSerialization::new(feature_socket, page_id.clone());

    let page_clone = page.clone();

    /*
    Now assume it returns an actual page serialization
    */
    mock_curator
        .expect_full_page()
        .times(1)
        .with(eq(page_id.clone()))
        .in_sequence(&mut seq)
        .return_once(move |_| Ok(page_clone));

    /*
    Create the command adapter
    */
    let mut command_adapter = WsCommandAdapter::new(mock_curator);

    /*
    Call full page using page id
    */
    let (ws_res, terminate) =
        command_adapter.consume_ws_command(WsCommand::FullPage(SimplePageCommand {
            page_id: page_id.clone(),
        }));

    /*
    First time round, it should return an error
    */
    assert_eq!(ws_res, WsResponse::Error);

    /*
    Terminate should always be false
    */
    assert_eq!(terminate, false);

    /*
    Call this puppy again
    */
    let (ws_res, terminate) =
        command_adapter.consume_ws_command(WsCommand::FullPage(SimplePageCommand {
            page_id: page_id.clone(),
        }));

    /*
    Now ws_res should be a full page response
    */
    assert_eq!(ws_res, WsResponse::FullPage { page });

    /*
    Terminate should always be false on this command
    */
    assert_eq!(terminate, false);
}
