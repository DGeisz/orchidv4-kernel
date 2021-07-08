use mockall::*;

/// Port for controlling the underlying page objects
#[automock]
pub trait PageControlPort {
    /// Get a clone of the id of the page
    fn get_id(&self) -> String;
}
