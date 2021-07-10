use crate::page::feature_tree::feature::feature_serialization::FeatureSerialization;
use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use crate::page::feature_tree::feature_tree_control_port::mock_feature_tree_control_port;
use crate::page::page_serialization::PageSerialization;
use crate::page::Page;

#[test]
pub fn test_serialize() {
    let mut mock_feature_tree = Box::new(mock_feature_tree_control_port());

    let socket_ser = FeatureSocketSerialization::new(FeatureSerialization::None);
    let ser_clone = socket_ser.clone();

    mock_feature_tree
        .expect_serialize()
        .times(1)
        .return_once(move || ser_clone);

    let page = Page::new(mock_feature_tree);

    let page_id = page.get_id();

    assert_eq!(
        page.serialize(),
        PageSerialization::new(socket_ser, page_id)
    );
}
