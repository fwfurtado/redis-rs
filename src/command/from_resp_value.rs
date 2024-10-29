use crate::command::Command;
use crate::resp::Value;

impl From<Value> for Command {
    fn from(value: Value) -> Self {
        match value {
            Value::Array(values) => match values.as_slice() {
                [Value::Bulk(command), Value::Bulk(key)]  if command.to_uppercase() == "GET" => Command::Get(key.to_string()),
                [Value::Bulk(command), Value::Bulk(key), Value::Bulk(value)]  if command.to_uppercase() == "SET" => Command::Set(key.to_string(), value.to_string()),
                [Value::Bulk(command), values @ ..]  if command.to_uppercase() == "MSET" => {
                    let mut pairs = Vec::new();

                    for chunk in values.chunks(2) {
                        if let [Value::Bulk(a), Value::Bulk(b)] = chunk {
                            pairs.push((a.to_string(), b.to_string()));
                        }
                    }
                    Command::MSet(pairs)
                }
                [Value::Bulk(command), values @ ..]  if command.to_uppercase() == "MGET" => {
                    let keys = values.iter().filter_map(
                        |v| match v {
                            Value::Bulk(key) => Some(key.to_string()),
                            _ => None
                        }
                    ).collect();

                    Command::MGet(keys)
                }
                _ => Command::Multiple(values.into_iter().map(Command::from).collect())
            }
            Value::Bulk(command) if command == "PING" => Command::Ping,
            unknown => Command::Error(format!("Unknown command: {:?}", unknown)),
        }
    }
}