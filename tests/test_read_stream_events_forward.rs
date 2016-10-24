extern crate time;
extern crate http_event_store as es;

use es::error::HesError::*;
use es::error::UserErrorKind::*;

#[macro_use]
mod support;

#[test]
fn attempt_to_read_nonexistent_stream() {
    let client = es::client::Client::default();
    let nonexistent_stream_name = "nonexistent";
    let result = client.read_stream_events_forward(&nonexistent_stream_name, 0, 1, true);

    assert_error!(UserError, StreamNotFound, result.unwrap_err());
}
