Environment
===========

```
$ uname -a
Linux blah 4.4.0-45-generic #66-Ubuntu SMP Wed Oct 19 14:12:37 UTC 2016 x86_64 x86_64 x86_64 GNU/Linux

$ rustc --version
rustc 1.10.0 (cfcb716cf 2016-07-03)
```

Run test Event Store sever
==========================

```
cd ~/bin/EventStore-OSS-Ubuntu-14.04-v3.8.1
./run-node.sh --db ./ESData --log ./logs
```

TODO
====

Find MAX value for `Expected Version`.

Handle HTTP status codes:
* InternalServerError
* ServiceUnavailable
* NotImplemented
* Unauthorized

Event related C# files:
* https://github.com/EventStore/EventStore/blob/08c2bdf7dcadd154cffa549d273e3a8e4673c5a1/src/EventStore.Core/Data/EventRecord.cs
* https://github.com/EventStore/EventStore/blob/08c2bdf7dcadd154cffa549d273e3a8e4673c5a1/src/EventStore.Core/Data/ResolvedEvent.cs
* https://github.com/EventStore/EventStore/blob/08c2bdf7dcadd154cffa549d273e3a8e4673c5a1/src/EventStore.ClientAPI/EventData.cs
* https://github.com/EventStore/EventStore/blob/08c2bdf7dcadd154cffa549d273e3a8e4673c5a1/src/EventStore.ClientAPI/EventReadResult.cs

TBD
===

The best way to transform a domain event to an http_event_store event
---------------------------------------------------------------------

ES Event is a trait:
```
pub trait Event {
    fn event_id(&self) -> uuid::Uuid;
    fn event_type(&self) -> &str;
    fn data(&self) -> Option<String>;
}

impl hes::write::Event for TaskCreated {
    fn event_id(&self) -> uuid::Uuid { self.event_id }
    fn event_type(&self) -> &str { "task-created" }
    fn data(&self) -> Option<String> { Some(format!(r#"{{ "name": "{}" }}"#, self.name)) }
}
```

ES event is a struct:
```
pub struct Event {
    event_id: uuid::Uuid,
    event_type: &str,
    data: Option<String>
}

impl From<TaskCreated> for hes::write::Event {
    fn from(t: TaskCreated) -> Self {
        hes::write::Event {
            event_id: t.event_id,
            event_type: "task-created",
            data: Some(format!(r#"{{ "name": "{}" }}"#, t.name))
        }
    }
}
```
