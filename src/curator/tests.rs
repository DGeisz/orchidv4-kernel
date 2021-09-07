use crate::curator::Curator;
use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::page_generator::page_generator_control::mock_page_generator_control;
use crate::page::page_serialization::PageSerialization;
use crate::page::Page;
use crate::utils::id_generator::mock_id_gen_control;
use std::rc::Rc;
use uuid::Uuid;

#[test]
fn test_new_page_get_page() {
    /* Create the mock page generator */
    let mut mock_page_generator = mock_page_generator_control();

    /* Create a mock id generator */
    let mut mock_id_generator = mock_id_gen_control();

    let dec_id = Uuid::new_v4().to_hyphenated().to_string();

    mock_id_generator
        .expect_gen_id()
        .return_const(dec_id.clone());

    let mut new_page = Page::new(Rc::new(Box::new(mock_id_generator)));
    let page_id = new_page.get_id().clone();

    mock_page_generator
        .expect_generate_new_page()
        .return_once_st(move || new_page);

    /* Create the actual curator */
    let mut curator = Curator::new(Box::new(mock_page_generator));

    let expected_serialization =
        PageSerialization::new(page_id.clone(), vec![DecSocketSer::new(dec_id, None)]);

    assert_eq!(curator.new_page(), expected_serialization);

    /* Now test getting the page */
    assert_eq!(
        curator.get_page(page_id.clone()),
        Some(expected_serialization)
    );

    /* Now test getting a page that doesn't exist */
    assert_eq!(curator.get_page("random_id".to_string()), None);
}
