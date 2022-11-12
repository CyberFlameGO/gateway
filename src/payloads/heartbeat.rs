use crate::op::OpCode;

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
    /// use heartbeat::Heartbeat;
    /// let hb = Heartbeat::new("token");
    /// ```
    pub fn new(token: &str) -> Heartbeat {
        Heartbeat {
            op: OpCode::Heartbeat,
            token: Some(token.to_string())
        }
    }
}