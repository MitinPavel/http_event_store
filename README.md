# http_event_store

[Rust](https://www.rust-lang.org/en-US/) HTTP connector to [Event Store](https://geteventstore.com/).

Is currently targeting EventStore 3.x.

## Tags

Rust, Database, Event Store, Event Sourcing, CQRS, RESTful API, AtomPub
 
## Getting started

TODO: show `Cargo.toml`.

```
extern crate http_event_store;
extern crate serde_json;
extern crate uuid;

use http_event_store::client::Client;
use http_event_store::write::Event;
use http_event_store::read::BodyEntry;
use http_event_store::expected_version::ExpectedVersion;

fn main() {
    let client = Client::default(); // 127.0.0.1:2113 by default

    let event = Event {
        event_id: uuid::Uuid::new_v4(),
        event_type: "event-type".to_string(),
        data: Some(serde_json::from_str(r#"{ "a": "1" }"#).unwrap())
    };

    client.append_to_stream("newstream", ExpectedVersion::NoStream, vec![event]).unwrap();
    let stream = client.read_stream_events_forward::<BodyEntry>("newstream", 0, 100, true).unwrap();

    println!("Stream entry: {:?}", stream.entries[0]);
}
```

## Main public API types

* `client::Client` - API client
* `write::Event` - Event struct (write side)
* `read::Stream<E>`- Stream of events (read side)
* `read::BodyEntry` - Stream entry (read side)
* `error::ApiError` - API error

## API functions

### append_to_stream

cURL: `curl -i -d @simple-event.txt -H "Content-Type:application/vnd.eventstore.events+json" "http://127.0.0.1:2113/streams/newstream"`

Rust: `client.append_to_stream("newstream", ExpectedVersion::NoStream, vec![simple_event]);`

### read_stream_events_forward

cURL: `curl -i -H "Accept:application/vnd.eventstore.atom+json" "http://127.0.0.1:2113/streams/newstream"`

Rust: `let result = client.read_stream_events_forward::<NoneEntry>("newstream", 0, 100, true);`

###### Embedding data into stream

There are two options reading streams: embed full fledged events or just provide links to actual event data. Compare:
```
let stream_with_data = client.read_stream_events_forward::<BodyEntry>("newstream", 0, 100, true).unwrap();
let stream_with_links = client.read_stream_events_forward::<NoneEntry>("newstream", 0, 100, true).unwrap();

```
 
Check `tests/test_read_stream_events_forward.rs` and `src/lib.rs.in` for details.

### delete_stream

cURL: `curl -v -X DELETE http://127.0.0.1:2113/streams/foo`

Rust: `client.delete_stream("foo", ExpectedVersion::Any);`

### hard_delete_stream

cURL: `curl -v -X DELETE http://127.0.0.1:2113/streams/foo2 -H "ES-HardDelete:true"`

Rust: `client.hard_delete_stream("foo", ExpectedVersion::Any);`

## Development environment

```
$ uname -a
Linux blah 4.4.0-47-generic #68-Ubuntu SMP Wed Oct 26 19:39:52 UTC 2016 x86_64 x86_64 x86_64 GNU/Linux

$ rustc --version
rustc 1.13.0 (2c6933acc 2016-11-07)
```

## License

MIT
