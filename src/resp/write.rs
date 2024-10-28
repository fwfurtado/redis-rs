use std::io::{Write, Result};
use crate::resp::Value;

const CRLF_BYTES: &'static [u8] = b"\r\n";
const NULL_BYTES: &'static [u8] = b"$-1\r\n";
const NULL_ARRAY_BYTES: &'static [u8] = b"*-1\r\n";

impl Value {
    pub fn write(&self, mut writer: impl Write) -> Result<()> {
        let mut buffer = Vec::new();

        encode_to_buffer(&self, &mut buffer);

        writer.write_all(&buffer)
    }
}

fn encode_to_buffer(value: &Value, buffer: &mut Vec<u8>) {
    match value {
        Value::Null => buffer.extend_from_slice(NULL_BYTES),
        Value::NullArray => buffer.extend_from_slice(NULL_ARRAY_BYTES),
        Value::String(string) => write_string(buffer, b'+', string.clone()),
        Value::Error(error) => write_string(buffer, b'-', error.clone()),
        Value::Integer(number) => write_string(buffer, b':', number.to_string()),
        Value::Array(value) => {
            let size = value.len();

            buffer.push(b'*');
            buffer.extend_from_slice(size.to_string().as_bytes());
            buffer.extend_from_slice(CRLF_BYTES);

            for val in value {
                encode_to_buffer(val, buffer);
            }
        },
        Value::Bulk(string) => {
            let size = string.len();

            buffer.push(b'$');
            buffer.extend_from_slice(size.to_string().as_bytes());
            buffer.extend_from_slice(CRLF_BYTES);

            buffer.extend_from_slice(string.as_bytes());

            buffer.extend_from_slice(CRLF_BYTES);
        },
    }
}

#[inline]
fn write_string(buffer: &mut Vec<u8>, prefix: u8, value: String) {
    buffer.push(prefix);
    buffer.extend_from_slice(value.as_bytes());
    buffer.extend_from_slice(CRLF_BYTES);
}