extern crate time;
extern crate http_event_store as es;

#[test]
fn it_interacts_with_event_store() {
    let client = es::client::Client::new();
    let stream_name = test_stream_name();

    let raw_json = r#"[
                     {
                       "eventId": "50ed34a2-b26e-4610-8a2e-35ae8e63599e",
                       "eventType": "task-created",
                       "data": {
                         "name": "Initial Name"
                       }
                     },
                     {
                       "eventId": "26e9fdd6-d7fe-4acf-b8dc-a494300963dc",
                       "eventType": "task-renamed",
                       "data": {
                         "name": "Updated Name"
                       }
                     }
                   ]"#;

    client.append_to_stream(&stream_name, 987, raw_json.into());

    let stream = client.read_stream_events_forward(&stream_name, 0, 1, true).unwrap();

    println!("{:?}", stream)
}

#[test]
fn attempt_to_read_nonexistent_stream() {
    let client = es::client::Client::new();
    let nonexistent_stream_name = "nonexistent";

    let stream = client.read_stream_events_forward(&nonexistent_stream_name, 0, 1, true).unwrap();

     //    Ok(Response { status: NotFound, headers: Headers { Date: Tue, 20 Sep 2016 06:35:42 GMT, Keep-Alive: timeout=15,max=100, Content-Type: text/plain; charset=utf-8, Content-Length: 0, Access-Control-Allow-Methods: POST, DELETE, GET, OPTIONS, Server: Mono-HTTPAPI/1.0, Access-Control-Allow-Headers: Content-Type, X-Requested-With, X-Forwarded-Host, X-PINGOTHER, Authorization, ES-LongPoll, ES-ExpectedVersion, ES-EventId, ES-EventType, ES-RequiresMaster, ES-HardDelete, ES-ResolveLinkTo, Access-Control-Allow-Origin: *, Access-Control-Expose-Headers: Location, ES-Position, ES-CurrentVersion, }, version: Http11, url: "http://127.0.0.1:2113/streams/task-123456789?embed=body", status_raw: RawStatus(404, "Not Found"), message: Http11Message { is_proxied: false, method: None, stream: Wrapper { obj: Some(Reading(SizedReader(remaining=0))) } } })

    println!("{:?}", stream)
}

fn test_stream_name() -> String {
    format!("task-{}", time::get_time().sec)
}
