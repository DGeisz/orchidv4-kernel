use crate::curator::Curator;
use crate::page::page_generator::page_generator_control::mock_page_generator_control;
use crate::page::page_serialization::PageSerialization;
use crate::page::Page;

#[test]
fn test_new_page_get_page() {
    /* Create the mock page generator */
    let mut mock_page_generator = mock_page_generator_control();

    let mut new_page = Page::new();
    let page_id = new_page.get_id().clone();

    mock_page_generator
        .expect_generate_new_page()
        .return_once_st(move || new_page);

    /* Create the actual curator */
    let mut curator = Curator::new(Box::new(mock_page_generator));

    let expected_serialization = PageSerialization::new(page_id.clone(), vec![]);

    assert_eq!(curator.new_page(), expected_serialization);

    /* Now test getting the page */
    assert_eq!(
        curator.get_page(page_id.clone()),
        Some(expected_serialization)
    );

    /* Now test getting a page that doesn't exist */
    assert_eq!(curator.get_page("random_id".to_string()), None);
}
