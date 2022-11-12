use crate::op::OpCode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// Represents a `TERMINATE` payload.
pub struct Terminate {
    /// The operation code, defaults to `OpCode::Terminate`.
    pub op: OpCode,
    /// A set of data pertaining to the closure meaning.
    pub closure: Closure,
    /// Whether the connection can be reconnected to or not.
    pub healthy: bool,
    /// The internal UID related to the `TERMINATE` event.
    pub iuid: String
}

impl Terminate {
    /// Returns a `TERMINATE` payload with specified values.
    ///
    /// # Arguments
    ///
    /// * `closure` - A set of data pertaining to the closure meaning.
    /// * `healthy`? - Whether the connection can be reconnected to or not, defaults to `true`.
    /// * `iuid` - The internal UID related to the `TERMINATE` event.
    ///
    /// # Examples
    ///
    /// ```
    /// use gateway::Terminate;
    /// let declare = Terminate::new(/* Closure */, None, String::from("N/A."));
    /// ```
    pub fn new(closure: Closure, healthy: Option<bool>, iuid: String) -> Terminate {
        Terminate {
            op: OpCode::Terminate,
            closure,
            // for lazy eval use unwrap_or_else
            healthy: healthy.unwrap_or(true),
            iuid
        }
    }
}

/// Represents a connection closure within the Gateway.
#[derive(Serialize, Deserialize)]
pub struct Closure {
    /// The error code for the closure.
    pub code: u8,
    /// The reason the error occurred.
    pub message: Option<String>,
    /// Whether the Gateway or client made the closure occur.
    pub fault: ClosureType
}

impl Closure {
    /// Returns a `Closure` struct with specified values.
    ///
    /// # Arguments
    ///
    /// * `code` - The error code for the closure.
    /// * `message`? - The reason the error occurred, defaults to `"Unknown."`.
    /// * `fault` - Whether the Gateway or client made the closure occur.
    ///
    /// # Examples
    ///
    /// ```
    /// use gateway::{Closure, ClosureType};
    /// let new_close = Closure::new(4000, None, ClosureType::Gateway);
    /// ```
    pub fn new(code: u8, message: Option<String>, fault: ClosureType) -> Closure {
        Closure {
            code,
            message: Some(message.unwrap_or(String::from("Unknown."))),
            fault
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ClosureType {
    /// Caused by the Gateway, such as generic WebSocket closures.
    Gateway,
    /// Caused by the client, such as poorly timed HEARTBEAT responses.
    Client
}