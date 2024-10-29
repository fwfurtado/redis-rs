use std::string::ToString;
use lazy_static::lazy_static;
use crate::command::Command;
use crate::resp::Value;
use crate::storage;

lazy_static! {
    static ref OK: Value = Value::String("OK".to_string());
    static ref PONG: Value = Value::String("PONG".to_string());
}

impl Command {

    pub fn run(&self) -> Value {
        match self {
            Command::Multiple(commands) => {
                let values = commands
                    .iter()
                    .map(run_command)
                    .collect();

                Value::Array(values)
            }
            Command::Ping => PONG.clone(),
            Command::Error(msg) => Value::Error(msg.clone()),
            Command::Get(key) => storage::get(key).map_or(Value::Null, |value| Value::String(value.clone())),
            Command::Set(key, value) => { storage::set(key.clone(), value.clone()); OK.clone() }
            Command::MSet(values) => {
                storage::m_set(values.clone());
                OK.clone()
            }
            Command::MGet(keys) => {
                let read = storage::m_get(keys.clone());
                let values = read
                    .into_iter()
                    .map(|value| value.map_or(Value::Null, Value::String))
                    .collect();

                Value::Array(values)
            }
        }
    }
}

fn run_command(command: &Command) -> Value {
    command.run()
}


