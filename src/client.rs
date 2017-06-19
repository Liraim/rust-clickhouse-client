use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ClickhouseClient {
}

#[derive(Debug)]
pub struct ClientError {
    description: String
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Error for ClientError {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

pub fn http_client(host: &str, port: u16) -> Result<ClickhouseClient, ClientError> {
    unimplemented!();
}