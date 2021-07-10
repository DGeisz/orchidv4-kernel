use crate::page::feature_tree::feature::feature_serialization::FeatureSerialization;
use crate::page::feature_tree::feature_socket::feature_socket_control_port::mock_feature_socket_control;
use crate::page::feature_tree::feature_socket::feature_socket_serialization::FeatureSocketSerialization;
use crate::page::feature_tree::FeatureTree;
use std::rc::Rc;

#[test]
pub fn test_serialize() {
    let mut mock_socket = mock_feature_socket_control();

    let socket_ser = FeatureSocketSerialization::new(FeatureSerialization::None);
    let ser_clone = socket_ser.clone();

    /*
    We have to mock get id method in socket
    because the feature tree automatically
    adds the base socket to its socket reference
    */
    mock_socket
        .expect_get_id()
        .times(1)
        .return_once(|| String::from("socket_id"));

    /*
    Mock the serialize method, as that's what we'll primarily be using
    */
    mock_socket
        .expect_serialize()
        .times(1)
        .return_once(move || ser_clone);

    let feature_tree = FeatureTree::new(Rc::new(Box::new(mock_socket)));

    assert_eq!(socket_ser, feature_tree.serialize());
}
