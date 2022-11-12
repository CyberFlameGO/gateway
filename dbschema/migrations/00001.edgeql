CREATE MIGRATION m13gsnvdguiuq5zvn66z2tquleart2gntbzwj3mxw2r77zayxti3qa
    ONTO initial
{
  CREATE MODULE connection IF NOT EXISTS;
  CREATE MODULE payload IF NOT EXISTS;
  CREATE FUTURE nonrecursive_access_policies;
  CREATE SCALAR TYPE connection::ClosureType EXTENDING enum<Gateway, Client>;
  CREATE TYPE connection::Closure {
      CREATE REQUIRED PROPERTY code -> std::bytes;
      CREATE PROPERTY fault -> connection::ClosureType {
          SET default := (connection::ClosureType.Gateway);
      };
      CREATE PROPERTY message -> std::str;
  };
  CREATE TYPE connection::Connection {
      CREATE REQUIRED PROPERTY heartbeat_interval -> std::decimal;
      CREATE PROPERTY iuid -> std::uuid;
      CREATE PROPERTY last_ack -> std::datetime;
      CREATE REQUIRED PROPERTY token -> std::str;
  };
  CREATE SCALAR TYPE payload::OpCode EXTENDING enum<Hello, Heartbeat, HeartbeatAck, Terminate, Declare>;
  CREATE ABSTRACT TYPE payload::Payload {
      CREATE REQUIRED PROPERTY op -> payload::OpCode {
          CREATE CONSTRAINT std::one_of(payload::OpCode.Hello, payload::OpCode.Heartbeat, payload::OpCode.HeartbeatAck, payload::OpCode.Terminate, payload::OpCode.Declare);
      };
  };
  CREATE TYPE payload::Terminate EXTENDING payload::Payload {
      CREATE REQUIRED MULTI LINK closure -> connection::Closure;
      CREATE REQUIRED LINK iuid -> connection::Connection;
      CREATE REQUIRED PROPERTY healthy -> std::bool;
  };
  CREATE ABSTRACT TYPE default::State {
      CREATE REQUIRED MULTI LINK connections -> connection::Connection {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY release_status -> std::str;
      CREATE REQUIRED PROPERTY version -> std::bigint;
  };
  CREATE TYPE default::Server EXTENDING default::State {
      CREATE REQUIRED PROPERTY online -> std::bool;
  };
  CREATE TYPE payload::TrackFields {
      CREATE PROPERTY app -> std::str;
      CREATE PROPERTY os -> std::str;
  };
  CREATE TYPE payload::Declare EXTENDING payload::Payload {
      CREATE REQUIRED LINK tracker -> payload::TrackFields;
      CREATE REQUIRED PROPERTY token -> std::str;
  };
  CREATE TYPE payload::Heartbeat EXTENDING payload::Payload {
      CREATE PROPERTY token -> std::str;
  };
  CREATE TYPE payload::HeartbeatAck EXTENDING payload::Payload {
      CREATE REQUIRED PROPERTY last_call -> std::float32;
      CREATE PROPERTY latency -> std::float32;
  };
  CREATE TYPE payload::Hello EXTENDING payload::Payload {
      CREATE REQUIRED PROPERTY heartbeat_interval -> std::decimal {
          CREATE CONSTRAINT std::max_value(60);
          CREATE CONSTRAINT std::min_ex_value(30);
      };
      CREATE REQUIRED PROPERTY version -> std::str {
          CREATE CONSTRAINT std::regexp(r'v\d');
      };
  };
};
