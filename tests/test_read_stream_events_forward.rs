extern crate time;
extern crate http_event_store as es;

use es::error::HesError::*;
use es::error::UserErrorKind::*;

#[test]
fn attempt_to_read_nonexistent_stream() {
    let client = es::client::Client::new();
    let nonexistent_stream_name = "nonexistent";
    let result = client.read_stream_events_forward(&nonexistent_stream_name, 0, 1, true);

    match result {
        Err(e) => match e {
            UserError(user_error) => {
                match user_error {
                    StreamNotFound => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        },
        _ => assert!(false)
    }
}
