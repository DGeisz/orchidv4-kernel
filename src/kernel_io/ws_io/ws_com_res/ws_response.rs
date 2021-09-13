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
    DecSocket {
        page_id: String,
        res: DecSocketRes,
    },
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum DecSocketRes {
    Update {
        dec_socket_ser: DecSocketSer,
    },
    Append {
        dec_socket_ser: DecSocketSer,
    },
    Insert {
        rel_socket_id: String,
        before_rel: bool,
        dec_socket_ser: DecSocketSer,
    },
    Delete {
        dec_socket_id: String,
    },
}
