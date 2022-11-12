use crate::op::OpCode;
use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// Represents a `HELLO` payload.
pub struct Hello {
    /// The operation code, defaults to `OpCode::Hello`.
    pub op: OpCode,
    /// The amount of time a heartbeat should be sent in seconds.
    pub heartbeat_interval: u8,
    /// The current version of the Gateway, e.g. "v1"
    pub version: String
}

impl Default for Hello {
    fn default() -> Hello {
        Hello {
            op: OpCode::Hello,
            // We're going to make this purely random for now - no relation
            // to traffic flow.
            heartbeat_interval: thread_rng().gen_range(30..60),
            version: String::from("v1")
        }
    }
}