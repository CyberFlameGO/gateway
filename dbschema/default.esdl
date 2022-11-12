module default {
    # Represents the state of the Gateway, e.g. general details.
    abstract type State {
        # The current version of the Gateway.
        required property version -> bigint;

        # The release status of the Gateway.
        required property release_status -> str;

        # The connections that are ongoing within the Gateway.
        required multi link connections -> connection::Connection {
            constraint exclusive;
        };
    }

    # Represents the Gateway as a webserver in whole.
    type Server extending State {
        # Whether the server is currently up or not.
        required property online -> bool;
    }
}
