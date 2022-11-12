use serde::{Serialize, Deserialize};
use crate::op::OpCode;

#[derive(Serialize, Deserialize)]
/// Represents a `HEARTBEAT` payload.
pub struct Heartbeat {
    /// The operation code, defaults to `OpCode::Heartbeat`.
    pub op: OpCode,
    /// The authentication token of the connection.
    pub token: Option<String>
}

impl Heartbeat {
    /// Returns a `HEARTBEAT` payload with the token specified, if given.
    ///
    /// # Arguments
    ///
    /// * `token` - The authentication token of the connection.
    ///
    /// # Examples
    ///
    /// ```
    /// use gateway::Heartbeat;
    /// let hb = Heartbeat::new(None);
    /// ```
    pub fn new(token: Option<String>) -> Heartbeat {
        Heartbeat {
            op: OpCode::Heartbeat,
            token
        }
    }
}

#[derive(Serialize, Deserialize)]
/// Represents a `HEARTBEAT_ACK` payload.
pub struct HeartbeatAck {
    /// The time it took to process the last `HEARTBEAT` response in seconds.
    pub last_call: f32,
    ///	The current internal latency of the Gateway in seconds.
    pub latency: f32
}

impl HeartbeatAck {
    /// Returns a `HEARTBEAT_ACK` payload.
    ///
    /// # Arguments
    ///
    /// * `last_call` - The time it took to process the last `HEARTBEAT`
    ///   response in seconds.
    /// * `latency` - The current internal latency of the Gateway in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use gateway::HeartbeatAck;
    /// let hb_ack = HeartbeatAck::new(42.5, 0.18);
    /// ```
    pub fn new(last_call: f32, latency: f32) -> HeartbeatAck {
        HeartbeatAck { last_call, latency }
    }
}