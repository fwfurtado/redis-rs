use std::io::{BufRead, BufReader, Read};
use anyhow::{anyhow, Result};
use log::debug;
use crate::resp::{Error, Value};

const RESP_MAX_SIZE: i64 = 512 * 1024 * 1024;
const MINI_BUFFER_SIZE: usize = 3;

struct Reader<R> {
    buffer: BufReader<R>,
}

impl <R: Read> Reader<R> {
    fn new(reader: BufReader<R>) -> Self {
        Self {
            buffer: reader,
        }
    }

    fn read(&mut self) -> Result<Value> {
        let mut res = Vec::new();
        self.buffer.read_until(b'\n', &mut res)
            .map_err(|e| anyhow!(Error::BufReadError).context(e))?;

        let len = res.len();

        if len == 0 {
            return Err(anyhow!(Error::UnexpectedEof));
        }

        if len < MINI_BUFFER_SIZE {
            return Err(anyhow!(Error::InvalidInput("Buffer too short")));
        }

        if has_invalid_crlf(res[len-2], res[len-1]) {
            return Err(anyhow!(Error::InvalidInput("Invalid CRLF")));
        }

        let identifier = res[0];
        let bytes = res[1..len - 2].as_ref();

        match identifier {
            b'+' => parse_string(bytes).map(Value::String),
            b'-' => parse_string(bytes).map(Value::Error),
            b':' => parse_number(bytes).map(Value::Integer),
            b'*' => self.parse_array(bytes),
            b'$' => self.parse_bulk(bytes),
            _ => {
                unimplemented!("Invalid byte identifier: {}", identifier as char);
            }
        }
    }

    fn parse_bulk(&mut self, reader: &[u8]) -> Result<Value> {
        let number = parse_number(reader)?;

        if number == -1 {
            return Ok(Value::Null);
        }

        if is_out_of_resp_range(number) {
            return Err(anyhow!(Error::InvalidData("Invalid Bulk size")));
        }

        let mut buffer = Vec::new();

        let size = number as usize;
        buffer.resize(size + 2, 0);

        self.buffer
            .read_exact(buffer.as_mut_slice())
            .map_err(|e| anyhow!(Error::Io("Cannot read bulk")).context(e))?;

        if has_invalid_crlf(buffer[size], buffer[size + 1]) {
            return Err(anyhow!(Error::InvalidInput("Invalid CRLF")));
        }

        buffer.truncate(size);

        let value = parse_string(&buffer).map(Value::Bulk);

        debug!("Bulk: {:?}", value);

        value
    }


    fn parse_array(&mut self, reader: &[u8]) -> Result<Value> {
        let size = parse_number(reader)?;

        if size == -1 {
            return Ok(Value::NullArray);
        }

        if is_out_of_resp_range(size) {
            return Err(anyhow!(Error::InvalidData("Invalid Array size")));
        }

        let mut values = Vec::with_capacity(size as usize);

        for _ in 0..size {
            match self.read() {
                Ok(val) => values.push(val),
                Err(e) => return Err(e),
            }
        }

        debug!("Array: {:?}", values);

        Ok(Value::Array(values))
    }
}

#[inline]
fn parse_number(bytes: &[u8]) -> Result<i64> {
    let str_number = parse_string(bytes)?;
    str_number.parse::<i64>()
        .map_err(|e| anyhow!(Error::InvalidData("Invalid Integer")).context(e))
}

#[inline]
fn parse_string(bytes: &[u8]) -> Result<String> {
    String::from_utf8(bytes.to_vec())
        .map_err(|e| anyhow!(Error::InvalidData("Invalid Simple String")).context(e))
}

#[inline]
fn is_out_of_resp_range(number: i64) -> bool {
    number < 0 || number > RESP_MAX_SIZE
}

#[inline]
fn has_invalid_crlf(a: u8, b: u8) -> bool {
    a != b'\r' || b != b'\n'
}

impl Value {
    pub fn read(buffer: BufReader<&[u8]>) -> Result<Value> {
        let mut parser = Reader::new(buffer);

        parser.read()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_invalid_crlf() {
        let buffer = vec![b'+', b'P', b'O', b'N', b'G', b'\r', b'\n'];
        let len = buffer.len();
        assert!(!has_invalid_crlf(buffer[len-2], buffer[len-1]));
    }

    #[test]
    fn test_has_valid_crlf() {
        let buffer = vec![b'+', b'P', b'O', b'N', b'G', b'\n'];
        let len = buffer.len();
        assert!(has_invalid_crlf(buffer[len-2], buffer[len-1]));
    }

    #[test]
    fn test_parse_simple_ping() {
        let buffer: BufReader<&[u8]> = BufReader::new(b"+PING\r\n");

        let result = Value::read(buffer);

        assert!(result.is_ok());

        let command = result.unwrap();

        assert_eq!(command, Value::String("PING".to_string()));
    }

    #[test]
    fn parse_array_ping() {
        let buffer: BufReader<&[u8]> = BufReader::new(b"*1\r\n$4\r\nPING\r\n");

        let result = Value::read(buffer);

        assert!(result.is_ok());

        let command = result.unwrap();

        assert_eq!(command, Value::Array(vec![Value::Bulk("PING".to_string())]));
    }
}