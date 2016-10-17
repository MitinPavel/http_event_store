#[derive(Debug)]
pub enum ExpectedVersion {
    Number(u64),
    Empty,             //  0 states that the stream should exist but should be empty.
    NotExist,          // -1 states that the stream should not exist at the time of the writing (this write will create it).
    NoConflict,        // -2 states that this write should never conflict with anything and should always succeed.
    Exist,             // -4 to say that the stream should exist with any number of events in it.

    Unexpected(String) // Any string (including -3 OR less than or equal -5).
}

impl From<ExpectedVersion> for String {
    fn from(version: ExpectedVersion) -> String {
        match version {
            ExpectedVersion::Number(n) => n.to_string(),
            ExpectedVersion::Empty => "0".to_string(),
            ExpectedVersion::NotExist => "-1".to_string(),
            ExpectedVersion::NoConflict => "-2".to_string(),
            ExpectedVersion::Exist => "-4".to_string(),
            ExpectedVersion::Unexpected(s) => s,
        }
    }
}

impl From<String> for ExpectedVersion {
    fn from(string: String) -> ExpectedVersion {
        match string.as_ref() {
            "-4" => ExpectedVersion::Exist,
            "-2" => ExpectedVersion::NoConflict,
            "-1" => ExpectedVersion::NotExist,
            "0" => ExpectedVersion::Empty,
            _ => ExpectedVersion::parse_number_or_unexpected(string)
        }
    }
}

impl ExpectedVersion {
    fn parse_number_or_unexpected(string: String) -> ExpectedVersion {
        match string.parse::<u64>() {
            Ok(n) => if n >= 0 { ExpectedVersion::Number(n) } else { ExpectedVersion::Unexpected(string) },
            Err(_) => ExpectedVersion::Unexpected(string)
        }
    }
}
