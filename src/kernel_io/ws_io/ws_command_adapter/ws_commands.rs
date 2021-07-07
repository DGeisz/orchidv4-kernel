use serde::{Deserialize, Serialize};

/// This is a serializable command digestible by ws_io
/// for adaption to the curator control port.
///
/// Most commands have a direct controller counterpart
#[derive(Deserialize, Serialize)]
pub enum WsCommand {
    /*
    First we have basic editor commands
    */
    // INSERT_CHAR(PageCommandWithContent),
    // DELETE(SimplePageCommand),
    // ENTER(SimplePageCommand),
    // SHIFT_NEXT(SimplePageCommand),
    // SHIFT_PREVIOUS(SimplePageCommand),
    // SHIFT_DOWN(SimplePageCommand),
    // SHIFT_UP(SimplePageCommand),
    // RELEASE_FOCUS(SimplePageCommand),
    // TAKE_FOCUS(SimplePageCommand),
    // TOGGLE_PARENTHESIS(SimplePageCommand),
    // TERM_COMMIT(SimplePageCommand),
    // SAVE_FILE(SimplePageCommand),
    /*
    Basic page commands
    */
    NewPage,
}

#[derive(Deserialize, Serialize)]
pub struct SimplePageCommand {
    page_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct PageCommandWithContent {
    page_id: String,
    content: String,
}
