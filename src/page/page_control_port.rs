/// Port for controlling the underlying page objects
pub trait PageControlPort {
    /// Get a clone of the id of the page
    fn get_id(&self) -> String;
}
