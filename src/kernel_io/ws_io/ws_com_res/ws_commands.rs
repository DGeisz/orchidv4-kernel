use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub enum WsCommand {
    /// Instruction to create a new page. Also includes
    /// the id of the client from which this message originated
    NewPage {
        target_client: String,
    },
    FullPage {
        page_id: String,
    },
    FillDecSocket {
        page_id: String,
        socket_id: String,
        dec_name: String,
    },
    AppendDecSocket {
        page_id: String,
    },
    DeleteDecSocket {
        page_id: String,
        socket_id: String,
    },
}
