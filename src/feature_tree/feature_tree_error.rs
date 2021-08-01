use crate::feature_tree::feature::feature_request::FeatureRequest;

pub enum FeatureTreeError {
    SocketNotFound(u128),
    BadFeatureRequest(FeatureRequest),
    RequestNotCompatible(FeatureRequest, u128),
    SocketAlreadyBound(u128),
    SocketAlreadyEmpty(u128),
    SocketCannotRelease(u128),
    NewFeatureDoesNotHaveEmptySockets(FeatureRequest, u128),
    NoUpperFeatureFoundForRebind,
    RebindIncompatible,
    ChildFeatureInUse(u128),
}
