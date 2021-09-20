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
    DecSocket {
        page_id: String,
        cmd: DecSocketCommand,
    },
    TermDefSocket {
        page_id: String,
        cmd: TermDefSocketCommand,
    },
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub enum DecSocketCommand {
    Fill {
        socket_id: String,
        dec_name: String,
    },
    Append,
    Delete {
        socket_id: String,
    },
    DeleteContents {
        socket_id: String,
    },
    Insert {
        rel_socket_id: String,
        before_rel: bool,
    },
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub enum TermDefSocketCommand {
    Fill { tds_id: String, term_seq: String },
    DeleteContents { tds_id: String },
    SetRep { tds_id: String, rep: String },
}
