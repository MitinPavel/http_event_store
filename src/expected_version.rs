#[derive(Debug)]
pub enum ExpectedVersion {
    Number(i32),
    Empty,              // The stream should exist but should be empty.
    NoStream,           // The stream should not exist at the time of the writing.
    Any,                // This write should never conflict with anything and should always succeed.
    Exist,              // The stream should exist with any number of events in it.

    Unexpected(String), // Any string (including -3 OR less than or equal -5).
    Invalid,            // Just for compatibility with related C# ExpectedVersion class.
}

impl From<ExpectedVersion> for String {
    fn from(version: ExpectedVersion) -> String {
        match version {
            ExpectedVersion::Number(n) => n.to_string(),
            ExpectedVersion::Empty => "0".to_string(),
            ExpectedVersion::NoStream => "-1".to_string(),
            ExpectedVersion::Any => "-2".to_string(),
            ExpectedVersion::Invalid => "-3".to_string(),
            ExpectedVersion::Exist => "-4".to_string(),
            ExpectedVersion::Unexpected(s) => s,
        }
    }
}

impl From<String> for ExpectedVersion {
    fn from(string: String) -> ExpectedVersion {
        match string.as_ref() {
            "-4" => ExpectedVersion::Exist,
            "-3" => ExpectedVersion::Invalid,
            "-2" => ExpectedVersion::Any,
            "-1" => ExpectedVersion::NoStream,
            "0" => ExpectedVersion::Empty,
            _ => ExpectedVersion::parse_number_or_unexpected(string)
        }
    }
}

impl ExpectedVersion {
    fn parse_number_or_unexpected(string: String) -> ExpectedVersion {
        match string.parse::<i32>() {
            Ok(n) =>  ExpectedVersion::Number(n),
            Err(_) => ExpectedVersion::Unexpected(string)
        }
    }
}
