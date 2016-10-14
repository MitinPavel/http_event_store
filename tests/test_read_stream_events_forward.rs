extern crate time;
extern crate http_event_store as es;

#[test]
fn attempt_to_read_nonexistent_stream() {
    let client = es::client::Client::new();
    let nonexistent_stream_name = "nonexistent";
    let result = client.read_stream_events_forward(&nonexistent_stream_name, 0, 1, true);

    match result {
        Err(e) => match e {
            es::error::ApiError::ClientError(client_error) => {
                match client_error {
                    es::error::ClientError::StreamNotFound => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        },
        _ => assert!(false)
    }
}
