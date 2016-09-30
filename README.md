Environment
===========

```
$ uname -a
Linux blah 4.2.0-42-generic #49-Ubuntu SMP Tue Jun 28 21:26:26 UTC 2016 x86_64 x86_64 x86_64 GNU/Linux

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

* Find MAX value for `Expected Version`.

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

impl es::event::Event for TaskCreated {
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

impl From<TaskCreated> for es::event::Event {
    fn from(t: TaskCreated) -> Self {
        es::event::Event {
            event_id: t.event_id,
            event_type: "task-created",
            data: Some(format!(r#"{{ "name": "{}" }}"#, t.name))
        }
    }
}
```
