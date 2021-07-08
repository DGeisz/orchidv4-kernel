pub enum PageError {
    /// Error indicates that a particular page
    /// wasn't found.  Returns the id of the
    /// page requested
    PageNotFound(String),
}
