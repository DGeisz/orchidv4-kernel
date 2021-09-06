use crate::curator::curator_control::mock_curator_control;
use crate::kernel_io::ws_io::ws_com_res::ws_commands::WsCommand;
use crate::kernel_io::ws_io::ws_com_res::ws_response::WsResponse;
use crate::kernel_io::ws_io::ws_command_adapter::WsCommandAdapter;
use crate::page::page_serialization::PageSerialization;

#[test]
fn test_new_page() {
    /*
    Create mock curator
    */
    let mut mock_curator = mock_curator_control();

    let page_serialization = PageSerialization::new("id".to_string());

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
