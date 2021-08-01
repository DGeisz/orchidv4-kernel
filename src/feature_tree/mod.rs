use crate::feature_tree::feature::feature_generator::feature_generator_control::FeatureGeneratorControl;
use crate::feature_tree::feature::feature_request::FeatureRequest;
use crate::feature_tree::feature_binding::FeatureBinding;
use crate::feature_tree::feature_socket::socket_control::SocketControl;
use crate::feature_tree::feature_tree_control::FeatureTreeControl;
use crate::feature_tree::feature_tree_diff::FeatureTreeDiff;
use crate::feature_tree::feature_tree_error::FeatureTreeError;
use std::rc::Rc;

pub mod compact_feature;
pub mod feature;
pub mod feature_binding;
pub mod feature_socket;
pub mod feature_tree_control;
pub mod feature_tree_diff;
pub mod feature_tree_error;
pub mod feature_type;
pub mod feature_utils;

struct FeatureTree {
    base_socket: Rc<dyn SocketControl>,
    feature_generator: Rc<dyn FeatureGeneratorControl>,
}

impl FeatureTreeControl for FeatureTree {
    fn create_and_append(
        &self,
        feature_request: FeatureRequest,
        socket_id: u128,
    ) -> Result<FeatureTreeDiff, FeatureTreeError> {
        /*
        First, find the socket to which this request refers
        */
        let socket = if let Some(socket) = self.base_socket.get_socket_by_id(socket_id) {
            socket
        } else {
            return Err(FeatureTreeError::SocketNotFound(socket_id));
        };

        /*
        Make sure socket isn't already filled
        */
        if let Some(_) = socket.get_feature() {
            return Err(FeatureTreeError::SocketAlreadyBound(socket_id));
        }

        /*
        Next, attempt to use the feature generator and socket to generate
        a compatible feature for this socket
        */
        let feature = match self
            .feature_generator
            .generate_feature(&feature_request, &socket)
        {
            Some(feature) => feature,
            None => {
                /*
                If the block generator didn't find anything, then the
                request is incompatible
                */
                return Err(FeatureTreeError::RequestNotCompatible(
                    feature_request,
                    socket_id,
                ));
            }
        };

        /*
        Now then, bind the socket to the feature
        */
        FeatureBinding::bind(&feature, &socket);

        /*
        Return response indicating that a feature was
        appended to the feature tree
        */
        Ok(FeatureTreeDiff::Append(socket_id, feature.serialize()))
    }

    // fn create_and_insert(
    //     &self,
    //     feature_request: FeatureRequest,
    //     socket_id: u128,
    //     rebind_to_first: bool,
    // ) -> Result<FeatureTreeDiff, FeatureTreeError> {
    //     /*
    //     First find the socket with socket_id.
    //     Make sure socket it already bound
    //     */
    //     let old_socket = if let Some(old_socket) = self.base_socket.get_socket_by_id(socket_id) {
    //         old_socket
    //     } else {
    //         return Err(FeatureTreeError::SocketNotFound(socket_id));
    //     };
    //
    //     let old_feature = if let Some(feature) = old_socket.get_feature() {
    //         feature
    //     } else {
    //         return Err(FeatureTreeError::SocketNotFound(socket_id));
    //     };
    //
    //     let new_feature = match self
    //         .feature_generator
    //         .generate_feature(&feature_request, &old_socket)
    //     {
    //         Some(feature) => feature,
    //         None => {
    //             return Err(FeatureTreeError::RequestNotCompatible(
    //                 feature_request,
    //                 socket_id,
    //             ))
    //         }
    //     };
    //
    //     /*
    //     Attempt to find empty socket on this new feature
    //     */
    //     let new_socket = if rebind_to_first {
    //         if let Some(socket) = new_feature.first_unbound_socket() {
    //             socket
    //         } else {
    //             return Err(FeatureTreeError::NewFeatureDoesNotHaveEmptySockets(
    //                 feature_request,
    //                 socket_id,
    //             ));
    //         }
    //     } else {
    //         if let Some(socket) = new_feature.last_unbound_socket() {
    //             socket
    //         } else {
    //             return Err(FeatureTreeError::NewFeatureDoesNotHaveEmptySockets(
    //                 feature_request,
    //                 socket_id,
    //             ));
    //         }
    //     };
    //
    //     /*
    //     Make sure new feature and old socket are compatible
    //     */
    //     if !old_socket.is_compatible_with(&new_feature) {
    //         return Err(FeatureTreeError::RequestNotCompatible(
    //             feature_request,
    //             socket_id,
    //         ));
    //     }
    //
    //     /*
    //     If so, temporarily bind new feature and old socket
    //     */
    //     FeatureBinding::bind(&new_feature, &old_socket);
    //
    //     /*
    //     Now see if the new socket is compatible with the old feature
    //     */
    //     if !new_socket.is_compatible_with(&old_feature) {
    //         /*
    //         If not, rebind the old pair, indicate the request
    //         wasn't compatible
    //         */
    //         FeatureBinding::bind(&old_feature, &old_socket);
    //
    //         return Err(FeatureTreeError::RequestNotCompatible(
    //             feature_request,
    //             socket_id,
    //         ));
    //     }
    //
    //     /*
    //     If they're compatible, bind the second new pair
    //     */
    //     FeatureBinding::bind(&old_feature, &new_socket);
    //
    //     Ok(FeatureTreeDiff::Replace(socket_id, new_feature.serialize()))
    // }

    fn release_and_delete(&self, socket_id: u128) -> Result<FeatureTreeDiff, FeatureTreeError> {
        /*
        Find the socket
        */
        let socket = if let Some(socket) = self.base_socket.get_socket_by_id(socket_id) {
            socket
        } else {
            return Err(FeatureTreeError::SocketNotFound(socket_id));
        };

        /*
        Attempt to detach
        */
        if let Err(e) = socket.detach() {
            return Err(e);
        }

        /*
        Otherwise, indicate the binding was detached
        */
        Ok(FeatureTreeDiff::Detach(socket_id))
    }

    // fn remove_and_rebind(
    //     &self,
    //     socket_id: u128,
    //     rebind_to_first: bool,
    // ) -> Result<FeatureTreeDiff, FeatureTreeError> {
    //     /*
    //     First get the lower socket
    //     */
    //     let lower_socket = if let Some(socket) = self.base_socket.get_socket_by_id(socket_id) {
    //         socket
    //     } else {
    //         return Err(FeatureTreeError::SocketNotFound(socket_id));
    //     };
    //
    //     /*
    //     Get the feature in the socket, error if there isn't one
    //     */
    //     let lower_feature = if let Some(feature) = lower_socket.get_feature() {
    //         feature
    //     } else {
    //         return Err(FeatureTreeError::SocketAlreadyEmpty(socket_id));
    //     };
    //
    //     /*
    //     Get the upper socket associated with lower feature (based on rebind to first)
    //     */
    //     let upper_socket_option = if rebind_to_first {
    //         lower_feature.first_unbound_socket()
    //     } else {
    //         lower_feature.last_unbound_socket()
    //     };
    //
    //     let upper_socket = if let Some(socket) = upper_socket_option {
    //         socket
    //     } else {
    //         return Err(FeatureTreeError::NoUpperFeatureFoundForRebind);
    //     };
    //
    //     /*
    //     Get the upper feature in the socket
    //     */
    //     let upper_feature = if let Some(feature) = upper_socket.get_feature() {
    //         feature
    //     } else {
    //         return Err(FeatureTreeError::NoUpperFeatureFoundForRebind);
    //     };
    //
    //     /*
    //     Make sure the lower socket and upper feature
    //     are compatible
    //     */
    //     if !lower_socket.is_compatible_with(&upper_feature) {
    //         return Err(FeatureTreeError::RebindIncompatible);
    //     }
    //
    //     /*
    //     If so, bind the upper feature and lower socket
    //     (which also will deallocate the deleted feature)
    //     */
    //     FeatureBinding::bind(&upper_feature, &lower_socket);
    //
    //     Ok(FeatureTreeDiff::Rebind(socket_id, upper_feature.get_id()))
    // }
}
