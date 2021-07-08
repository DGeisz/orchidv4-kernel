use crate::curator::Curator;
use crate::page::page_control_port::mock_page_control_port;
use crate::page::page_generator::page_generator_port::mock_page_generator;

#[test]
pub fn test_new_page() {
    let page_id = String::from("page_id");
    let pid_clone = page_id.clone();

    let mut mock_page_generator = Box::new(mock_page_generator());
    let mut mock_page_control_port = Box::new(mock_page_control_port());

    mock_page_control_port
        .expect_get_id()
        .times(1)
        .return_once(move || page_id);

    mock_page_generator
        .expect_generate_page()
        .times(1)
        .return_once(move || mock_page_control_port);

    let mut curator = Curator::new(mock_page_generator);

    assert_eq!(curator.new_page(), pid_clone);
}
