pub enum ExpectedVersion {
    NoConflict, // -2 states that this write should never conflict with anything and should always succeed.
    NotExist,   // -1 states that the stream should not exist at the time of the writing (this write will create it)
    Empty,      //  0 states that the stream should exist but should be empty
    Number(u64)
}

impl From<ExpectedVersion> for String {
    fn from(version: ExpectedVersion) -> String {
        match version {
            ExpectedVersion::NoConflict => "-2".to_string(),
            ExpectedVersion::NotExist => "-1".to_string(),
            ExpectedVersion::Empty => "0".to_string(),
            ExpectedVersion::Number(n) => n.to_string()
        }
    }
}

