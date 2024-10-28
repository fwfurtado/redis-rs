use crate::resp::Value;


#[derive(Debug, PartialEq, Eq)]
pub enum Command{
    Ping,
    Pong
}

impl Command {
    pub fn run(&self) -> Value {
        match self {
            Command::Ping => Value::String("PONG".to_string()),
            Command::Pong => Value::String("PING".to_string()),
        }
    }
}


impl From<Value> for Command {
    fn from(value: Value) -> Self {
        match value {
            Value::Array(values) => match values.as_slice() {
                [Value::Bulk(command)] if command == "PING" => Command::Ping,
                [Value::Bulk(command)] if command == "PONG" => Command::Pong,
                _ => unimplemented!("Unknown command: {:?}", values),
            },
            unknown => unimplemented!("Unknown command: {:?}", unknown),
        }
    }
}