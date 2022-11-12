module connection {
    # Represents a connection within the Gateway.
    type Connection {
        # The authentication token of the connection.
        required property token -> str;

        # The given heartbeat interval for HEARTBEAT events and responses.
        required property heartbeat_interval -> decimal;

        # The internal, unique UID of the connection.
        # This UUID is relative to TERMINATE events.
        # Optional as a connection may not respond with a DECLARE after a
        # HELLO.
        property iuid -> uuid;

        # The date and time of the last acknowledged HEARTBEAT response.
        # A HEARTBEAT event will not be tracked.
        property last_ack -> datetime;
    }

    # Represents a connection closure within the Gateway.
    type Closure {
        # The error code for the closure.
        # Using bytes as EdgeDB do not recognise their own 8-bit integer.
        required property code -> bytes;

        # The reason the error occurred.
        property message -> str;

        # Whether the "gateway" or "client" made the closure occur.
        property fault -> ClosureType {
            default := ClosureType.Gateway;
        }
    }

    # Represents the types of Gateway closures.
    scalar type ClosureType extending enum<
        # Caused by the Gateway, such as generic WebSocket closures.
        Gateway,

        # Caused by the client, such as poorly timed HEARTBEAT responses.
        Client
    >;
}