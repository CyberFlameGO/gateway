use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// Represents the operation codes of the Gateway.
pub enum OpCode {
    /// HELLO - upon connecting to the Gateway.
    Hello,
    /// HEARTBEAT - upon request or needing to validate the connection.
    Heartbeat,
    /// HEARTBEAT_ACK - validates the last given HEARTBEAT response.
    HeartbeatAck,
    /// TERMINATE - upon failure to comply to specific conditions.
    Terminate,
    /// DECLARE - upon informing the Gateway of a client's identity.
    Declare
}