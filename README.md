# object-id

Rust unique object ID

A simple object which guaranties that object ID is unique until dropped.

## How it works

The **UniqueId** object does not generate any IDs itself, instead it allocates
a single byte in the heap. The pointer to the byte can be used to identify an
object itself and any other object which holds the ID one.

## Why it is useful

Unique object IDs are used (usually by Drop) to notify various 3rd party
objects that the source is dropped and e.g. must be unsubscribed from events,
removed from collections, maps etc.

## Cloning

Keep in mind that when either **UniqueId** object or the parent object is
cloned, the ID is changed.

## Other traits

**UniqueId** implements the majority of traits the parent object may demand.

## How to use

As the **UniqueId** object is changed when cloned, no matter solo or with the
parent object, the target collections should keep / compare it numeric
representation instead. Example:

```rust
use object_id::UniqueId;
use once_cell::sync::Lazy;
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Mutex;

static EVENT_RECEIVERS: Lazy<Mutex<Vec<(usize, Sender<()>)>>> = Lazy::new(<_>::default);

struct Client {
    id: UniqueId,
    rx: Receiver<()>
}

impl Drop for Client {
    fn drop(&mut self) {
        EVENT_RECEIVERS.lock().unwrap().retain(|(id, _)| *id != self.id.as_usize());
    }
}

{
    let (tx, rx) = mpsc::channel();
    let client = Client { id: <_>::default(), rx };
    EVENT_RECEIVERS.lock().unwrap().push((client.id.as_usize(), tx));
} // the client is dropped here
assert!(EVENT_RECEIVERS.lock().unwrap().is_empty());
```

The similar method can be used to store an async **Waker** of an object
together with its unique ID to notify the producer that the object Future is
dropped before data is consumed and properly handle aborts.
