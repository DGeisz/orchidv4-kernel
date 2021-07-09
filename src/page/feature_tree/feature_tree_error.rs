/// Error that arise in the feature tree
pub enum FeatureTreeError {
    /// Indicates the kernel tried to fill
    /// a feature socket that was already
    /// filled by an existing feature.  Provides
    /// the id of the socket in question
    SocketAlreadyFilled(String),

    /// Indicates we tried to fetch a non-existing
    /// socket.  Provides the id that was used to
    /// fetch the socket
    SocketDoesNotExist(String),
}
