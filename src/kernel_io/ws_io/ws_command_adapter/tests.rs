use crate::curator::curator_control::mock_curator_control;
use crate::kernel_io::ws_io::ws_com_res::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_com_res::ws_response::WsResponse;
use crate::kernel_io::ws_io::ws_command_adapter::WsCommandAdapter;
use crate::page::page_serialization::PageSerialization;
use mockall::predicate::eq;

#[test]
fn test_new_page() {
    /*
    Create mock curator
    */
    let mut mock_curator = mock_curator_control();

    let page_serialization = PageSerialization::new("id".to_string(), vec![]);

    mock_curator
        .expect_new_page()
        .return_const(page_serialization.clone());

    let mut ws_command_adapter = WsCommandAdapter::new(Box::new(mock_curator));

    let target_client = "target".to_string();

    assert_eq!(
        ws_command_adapter.consume_ws_command(WsCommand::NewPage {
            target_client: target_client.clone()
        }),
        (
            WsResponse::NewPage {
                target_client,
                new_page: page_serialization
            },
            false
        )
    )
}

#[test]
fn test_full_page() {
    /*
    Create mock curator
    */
    let mut mock_curator = mock_curator_control();

    let page1_id = "id".to_string();

    let page_serialization = PageSerialization::new(page1_id.clone(), vec![]);

    mock_curator
        .expect_get_page()
        .with(eq(page1_id.clone()))
        .return_const(Some(page_serialization.clone()));

    let page2_id = "nid".to_string();

    mock_curator
        .expect_get_page()
        .with(eq(page2_id.clone()))
        .return_const(None);

    let mut ws_command_adapter = WsCommandAdapter::new(Box::new(mock_curator));

    /*
    First test with an actual page.  Then test with a page that doesn't exist
    */
    assert_eq!(
        ws_command_adapter.consume_ws_command(WsCommand::FullPage { page_id: page1_id }),
        (
            WsResponse::FullPage {
                page: page_serialization
            },
            false
        )
    );

    assert_eq!(
        ws_command_adapter.consume_ws_command(WsCommand::FullPage { page_id: page2_id }),
        (WsResponse::Error, false)
    );
}
