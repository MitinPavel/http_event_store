header! { (ESCurrentVersion, "ES-CurrentVersion") => [String] }
header! { (ESExpectedVersion, "ES-ExpectedVersion") => [String] }
header! { (ESResolveLinkTos, "ES-ResolveLinkTos") => [bool] }
header! { (ESHardDelete, "ES-HardDelete") => [bool] }

pub mod append_to_stream;
pub mod read_stream_events_forward;
pub mod delete_stream;
mod to_error;
