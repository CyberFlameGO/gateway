use crate::op::OpCode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// Represents a `DECLARE` payload.
pub struct Declare {
    /// The operation code, defaults to `OpCode::Declare`.
    pub op: OpCode,
    /// An authentication token of the connection.
    pub token: String,
    /// Tracker fields for analytic purposes.
    pub tracker: Option<TrackFields>
}

impl Declare {
    /// Returns a `DECLARE` payload with specified values.
    ///
    /// # Arguments
    ///
    /// * `token` - An authentication token used for connection.
    /// * `tracker`? - Tracker fields for analytic purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// use gateway::Declare;
    /// let declare = Declare::new("token", None);
    /// ```
    pub fn new(token: &str, tracker: Option<TrackFields>) -> Declare {
        Declare {
            op: OpCode::Declare,
            token: token.to_string(),
            tracker
        }
    }
}

#[derive(Serialize, Deserialize)]
/// Represents a set of tracker fields for the `DECLARE` payload.
pub struct TrackFields {
    /// The operation code to use for the heartbeat.
    pub os: Option<String>,
    /// The name of the application's client.
    pub app: Option<String>
}

impl TrackFields {
    /// Returns a `TrackFields` struct with specified values.
    ///
    /// # Arguments
    ///
    /// * `os`? - The operating system of the client.
    /// * `app`? - The name of the application's client.
    ///
    /// # Examples
    ///
    /// ```
    /// use gateway::TrackFields;
    /// let os: &str = "Windows 10";
    /// let track_fields = TrackFields::new(Some(os.to_string()), None);
    /// ```
    pub fn new(os: Option<String>, app: Option<String>) -> TrackFields {
        TrackFields { os, app }
    }
}