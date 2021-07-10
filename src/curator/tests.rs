use crate::curator::curator_control_port::CuratorControlPort;
use crate::curator::Curator;
use crate::page::feature_tree::feature::feature_serialization::FeatureSerialization;
use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use crate::page::page_control_port::{mock_page_control_port, PageControlPort};
use crate::page::page_error::PageError;
use crate::page::page_generator::page_generator_port::{mock_page_generator, PageGeneratorPort};
use crate::page::page_serialization::PageSerialization;

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

#[test]
pub fn test_full_page() {
    let page_id = String::from("page_id");

    /*
    First we need to initialize the curator with a page
    */
    let mut mock_page_generator = Box::new(mock_page_generator());
    let mut mock_page_control_port = Box::new(mock_page_control_port());

    let pid_clone = page_id.clone();

    mock_page_control_port
        .expect_get_id()
        .times(1)
        .return_once(move || pid_clone);

    /*
    We need to mock the serialize method
    */
    let page: PageSerialization = PageSerialization::new(
        FeatureSocketSerialization::new(FeatureSerialization::None),
        page_id.clone(),
    );
    let page_clone = page.clone();

    mock_page_control_port
        .expect_serialize()
        .times(1)
        .return_once(move || page_clone);

    mock_page_generator
        .expect_generate_page()
        .times(1)
        .return_once(move || mock_page_control_port);

    let mut curator = Curator::new(mock_page_generator);
    curator.new_page();

    /*
    Ok, now we can actually do some testing.
    First, let's try to get a page that doesn't exist
    */
    let wrong_page = String::from("wrong_page");

    assert_eq!(
        curator.full_page(wrong_page.clone()),
        Err(PageError::PageNotFound(wrong_page.clone()))
    );

    /*
    Now let's attempt to fetch a page that does exist
    */
    assert_eq!(curator.full_page(page_id), Ok(page));
}
