# gateway

A purely experimental, prototype concept for a Gateway infrastructure in Rust
for Shadowbin.

### Understanding the documentation

There are a couple of symbols we use to explain the relationship of data
for some payload fields:

- `*` Means that this field is only present if it is received from the Gateway,
  indicating you've been requested to send an event. An `op` field will always
  be present in order to detect if the event requires a response. The symbol
  only applies for operation codes allowing a send and receive action. If the
  `*` symbol is used on an operation code with a payload that does not
  recognise both actions, this is instead recognised as a condition.
- `?` Signifies that a particular part of a field is optional. If this is
  placed on a type, this means that the data received may be `null`; and
  a non-required field otherwise for sending. A specified default value will
  *always* override the statement.

## Operation Codes

The Gateway uses an [`OpCode`](#operation-codes) enumerator to determine what
operations may take place for client and webserver connections:

Constant        | Code | Action       | Payload?
----------------|------|--------------|---------
`HELLO`         | `0`  | Receive      | [`Hello`](#hello)
`HEARTBEAT`     | `1`  | Send/Receive | [`Heartbeat`](#heartbeat)
`HEARTBEAT_ACK` | `2`  | Receive      | [`HeartbeatAck`](#heartbeat_ack)
`TERMINATE`     | `3`  | Receive      | [`Terminate`](#terminate)
`DECLARE`       | `4`  | Send         | [`Declare`](#declare)

## Payloads

Payloads represent two actions in the Gateway: an **event**, when one is
received, and a **response** when one may be sent to it.

Make sure to read [our documentation format](#understanding-the-documentation)
before proceeding any further.

### `HELLO`

A `HELLO` payload is sent as an event upon connecting to the Gateway server.
Each new connection will be met with this payload, containing information
useful to a client:

Field                | Type        | Description                                               | Default
---------------------|-------------|-----------------------------------------------------------|--------
`heartbeat_interval` | `u8`        | The amount of time a heartbeat should be sent in seconds. | ≅ 30
`version`            | `String`    | The current version of the Gateway, e.g. `"v1"`           |

### `HEARTBEAT`

A `HEARTBEAT` payload can either be sent to, or *received* from the Gateway.
These are the following conditions for either to occur:

- It is *received* when a client has failed to send a `HEARTBEAT` within
  the timespan of a `heartbeat_interval`. This interval correlates to a
  singular connection through a `token`.
- It is *sent* when a client needs to make a request after either receiving a
  `HEARTBEAT` or sending the payload as a packet before such requirement is
  met.

You **do not** need to send a `HEARTBEAT` when you have received a
`HEARTBEAT_ACK`. Failure to send a `HEARTBEAT` *before* the interval may
result in a follow-up `TERMINATE` event.

The data model of a `HEARTBEAT` payload is simple:

Field                | Type                         | Description                                  | Default
---------------------|------------------------------|----------------------------------------------|--------
*`op`?               | [`OpCode`](#operation-codes) | The operation code to use for the heartbeat. | `1`
`token`?             | `String`?                    | An authentication token for the connection.  |

### `HEARTBEAT_ACK`

A `HEARTBEAT_ACK` payload is sent as an event upon recognising a valid
`HEARTBEAT` response:

Field       | Type   | Description                                                  | Default
------------|--------|--------------------------------------------------------------|--------
`last_call` | `f32`  | The time it took to process the last `HEARTBEAT` in seconds. |
`latency`   | `f32`? | The current internal latency of the Gateway in seconds.      |

When one has been acknowledged, you *must* wait another `heartbeat_interval`
seconds before sending another `HEARTBEAT` response.

### `TERMINATE`

A `TERMINATE` payload is sent as an event upon a series of any met condition:

- A [`HEARTBEAT`](#heartbeat) was not sent within the timespan of a
`heartbeat_interval`.
- The Gateway lost track of the connection's [identity](#declare), or
had a connection hiccup.
- An abnormal or generic WebSocket closure occurred.

Field     | Type                  | Description                                          | Default
----------|-----------------------|------------------------------------------------------|--------
`closure` | [`Closure`](#closure) | A set of data pertaining to the closure meaning.     |
`healthy` | `bool`                | Whether the connection can be reconnected to or not. | `false`
`iuid`*   | `String`              | The internal UID related to the `TERMINATE` event.   |

\* This field is used for debugging purposes only. Do not use this field for
anything relating to production.

### `Closure`

A `Closure` is only present upon a `TERMINATE`. The error status within one
will also be accessible through a client connection's status:

Field      | Type      | Description                                                   | Default
-----------|-----------|---------------------------------------------------------------|--------
`code`*    | `u8`      | The error code for the closure.                               |
`message`* | `String`? | The reason the error occurred.                                |
`fault`    | `String`  | Whether the `"gateway"` or `"client"` made the closure occur. |

### `DECLARE`

A `DECLARE` payload is sent as an event to inform the Gateway that you wish to
make a persistent connection:

Field     | Type                          | Description                                  | Default
----------|-------------------------------|----------------------------------------------|--------
`token`   | `String`                      | An authentication token for the connection.  |
`tracker` | [`TrackFields`](#trackfields) | Tracker fields for analytic purposes.        |

**NOTE**: We avoid using the `HELLO` payload because we want to delineate user
data from server-specific data.

#### `TrackFields`

For applications that wish to opt-in to being tracked, they may also send
a set of `TrackFields`. Shadowbin will never store your IP address or any
personal information that is not specified otherwise:

Field  | Type     | Description                           | Default
-------|----------|---------------------------------------|--------
`os`?  | `String` | The operating system of the client.   |
`app`? | `String` | The name of the application's client. |