use std::{str::FromStr, string::ParseError};
/// Valid values for an HTTP method
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}

impl FromStr for Method {
    type Err = ParseError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        let input = input.to_ascii_uppercase();
        let input = input.trim();

        Ok(match input {
            "OPTIONS" => Method::Options,
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "TRACE" => Method::Trace,
            "HEAD" => Method::Head,
            "CONNECT" => Method::Connect,
            "PATCH" => Method::Patch,
            _ => Method::Get,
        })
    }
}
