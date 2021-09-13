use crate::page::lexicon::declaration::declaration_serialization::DecSocketSer;
use crate::page::page_serialization::PageSerialization;
use serde::{Deserialize, Serialize};

/// This is a serializable output that can be sent
/// back to the client
#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum WsResponse {
    Error,
    NewPage {
        target_client: String,
        new_page: PageSerialization,
    },
    FullPage {
        page: PageSerialization,
    },
    DecSocketUpdate {
        page_id: String,
        dec_socket_ser: DecSocketSer,
    },
    DecSocketAppend {
        page_id: String,
        dec_socket_ser: DecSocketSer,
    },
    DecSocketDelete {
        page_id: String,
        dec_socket_id: String,
    },
}
