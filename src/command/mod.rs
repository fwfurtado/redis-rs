mod from_resp_value;
mod run;

#[derive(Debug, PartialEq, Eq)]
pub enum Command{
    Ping,
    Get(String),
    Set(String, String),
    MGet(Vec<String>),
    MSet(Vec<(String, String)>),
    Multiple(Vec<Command>),
    Error(String),
}
