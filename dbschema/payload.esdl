module payload {
    # Represents the operation codes of the Gateway.
    scalar type OpCode extending enum<
        # HELLO - upon connecting to the Gateway.
        Hello,

        # HEARTBEAT - upon request or needing to validate the connection.
        Heartbeat,

        # HEARTBEAT_ACK - validates the last given HEARTBEAT response.
        HeartbeatAck,

        # TERMINATE - upon failure to comply to specific conditions.
        Terminate,

        # DECLARE - upon informing the Gateway of a client's identity.
        Declare
    >;

    # Represents a base payload.
    abstract type Payload {
        # The operation code of the payload.
        required property op -> OpCode {
            constraint one_of(
                OpCode.Hello,
                OpCode.Heartbeat,
                OpCode.HeartbeatAck,
                OpCode.Terminate,
                OpCode.Declare
            );
        };
    }

    # Represents a HELLO payload.
    type Hello extending Payload {
        # The amount of time a heartbeat should be sent in seconds.
        required property heartbeat_interval -> decimal {
            # The default will always be a minimum of 30 seconds.
            # We flexibly allow up to 1 minute because we can't
            # always predict the rate of traffic.
            constraint min_ex_value(30);
            constraint max_value(60);
        };

        # The current version of the Gateway, e.g. "v1"
        required property version -> str {
            constraint regexp(r"v\d");
        };
    }

    # Represents a HEARTBEAT payload.
    type Heartbeat extending Payload {
        # The authentication token of the connection.
        property token -> str;
    }

    # Represents a HEARTBEAT_ACK payload.
    type HeartbeatAck extending Payload {
        # The time it took to process the last HEARTBEAT response in seconds.
        required property last_call -> float32;

        # The current internal latency of the Gateway in seconds.
        property latency -> float32;
    }

    # Represents a TERMIANTE payload.
    type Terminate extending Payload {
        # A set of data pertaining the closure meaning.
        required multi link closure -> connection::Closure;

        # Whether the connection can be reconnected to or not.
        # When it cannot be reconnected, we force a 24-hour timeout in the
        # backend.
        required property healthy -> bool;

        # The internal UID related to the TERMINATE event.
        # This targets a given connection for simplicity.
        required link iuid -> connection::Connection;
    }

    # Represents a DECLARE payload.
    type Declare extending Payload {
        # An authentication token for the connection.
        required property token -> str;

        # Tracker fields for analytic purposes.
        required link tracker -> TrackFields;
    }

    # Represents a set of tracker fields for the DECLARE payload.
    type TrackFields {
        # The operating system of the client.
        property os -> str;

        # The anme of the application's client.
        property app -> str;
    }
}