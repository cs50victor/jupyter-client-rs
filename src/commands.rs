use errors::Result;
use header::Header;
use hmac::Mac;
use serde::{Serialize as SerdeSerialize, Serializer};
use serde_derive::Serialize;
use serde_json::json;
use std::collections::HashMap;
use wire::WireMessage;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Command {
    KernelInfo,
    Execute {
        code: String,
        silent: bool,
        store_history: bool,
        user_expressions: HashMap<String, String>,
        allow_stdin: bool,
        stop_on_error: bool,
    },
    Inspect {
        code: String,
        cursor_pos: u64,
        detail_level: DetailLevel,
    },
    Complete {
        code: String,
        cursor_pos: u64,
    },
    History {
        output: bool,
        raw: bool,
        hist_access_type: HistoryAccessType,
        unique: bool,
    },
    IsComplete {
        code: String,
    },
    Shutdown {
        restart: bool,
    },
}

impl Command {
    pub(crate) fn into_wire<M: Mac>(self, auth: M) -> Result<WireMessage<M>> {
        match self {
            Command::KernelInfo => {
                let header = Header::new("kernel_info_request");
                let header_bytes = header.to_bytes()?;
                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content: b"{}".to_vec(),
                    auth,
                })
            }
            r @ Command::Execute { .. } => {
                let header = Header::new("execute_request");
                let header_bytes = header.to_bytes()?;
                let content_str = serde_json::to_string(&r)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
            r @ Command::Inspect { .. } => {
                let header = Header::new("inspect_request");
                let header_bytes = header.to_bytes()?;
                let content_str = serde_json::to_string(&r)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
            r @ Command::Complete { .. } => {
                let header = Header::new("complete_request");
                let header_bytes = header.to_bytes()?;
                let content_str = serde_json::to_string(&r)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
            Command::History {
                output,
                raw,
                hist_access_type,
                unique,
            } => {
                let header = Header::new("history_request");
                let header_bytes = header.to_bytes()?;

                let content = match hist_access_type {
                    HistoryAccessType::Tail { n } => json!({
                        "output": output,
                        "raw": raw,
                        "unique": unique,
                        "hist_access_type": "tail",
                        "session": null,
                        "start": null,
                        "stop": null,
                        "n": n,
                        "pattern": null,
                    }),
                    HistoryAccessType::Range {
                        session,
                        start,
                        stop,
                    } => json!({
                            "output": output,
                            "raw": raw,
                            "unique": unique,
                            "hist_access_type": "tail",
                            "session": session,
                            "start": start,
                            "stop": stop,
                            "n": null,
                            "pattern": null,
                    }),
                    HistoryAccessType::Search { pattern } => json!({
                            "output": output,
                            "raw": raw,
                            "unique": unique,
                            "hist_access_type": "tail",
                            "session": null,
                            "start": null,
                            "stop": null,
                            "n": null,
                            "pattern": pattern,
                    }),
                };

                let content_str = serde_json::to_string(&content)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
            Command::IsComplete { code } => {
                let header = Header::new("is_complete_request");
                let header_bytes = header.to_bytes()?;

                let content_json = json!({
                    "code": code,
                });
                let content_str = serde_json::to_string(&content_json)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content: content,
                    auth,
                })
            }
            Command::Shutdown { restart } => {
                let header = Header::new("shutdown_request");
                let header_bytes = header.to_bytes()?;
                let content_json = json!({
                    "restart": restart,
                });
                let content_str = serde_json::to_string(&content_json)?;
                let content = content_str.into_bytes();

                Ok(WireMessage {
                    header: header_bytes.to_vec(),
                    parent_header: b"{}".to_vec(),
                    metadata: b"{}".to_vec(),
                    content,
                    auth,
                })
            }
        }
    }
}

#[derive(Serialize, Debug)]
pub enum HistoryAccessType {
    Tail { n: u64 },
    Range { session: i64, start: u64, stop: u64 },
    Search { pattern: String },
}

#[derive(Debug)]
pub enum DetailLevel {
    Zero,
    One,
}

impl SerdeSerialize for DetailLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DetailLevel::Zero => serializer.serialize_i32(0),
            DetailLevel::One => serializer.serialize_i32(1),
        }
    }
}
