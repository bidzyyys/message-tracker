# Message Tracker

This repository contains a trait called `MessageTracker` and a struct `MessageStore` that implements this trait.

The purpose of this code is to track a configurable fixed amount of messages in a first-in-first-out (FIFO) manner, ensuring that duplicate messages are not stored in the queue.

## MessageTracker Trait:
- `fn add(&mut self, message: Message)` -- Adds a message to the tracker, deleting the oldest message if necessary to maintain the configured FIFO size.

- `fn delete(&mut self, id: &str) -> Option<Message>` -- Deletes a message from the tracker based on its ID and returns the deleted message if found.

- `fn get(&self, id: &str) -> Option<Message>` -- Returns a message for a given ID. The message is retained in the tracker.

- `fn get_all(&self) -> Vec<Message>` -- Returns all messages in the tracker in FIFO order.

## MessageStore:
-  `fn new(fifo_size: usize) -> Self` -- Creates a new MessageStore instance with the specified FIFO size.

## Performance Analysis:

### Time Complexity
1. `fn add(&mut self, message: Message)`:
    - Enqueuing a message at the back of the queue has a time complexity of O(1) because it's implemented using a VecDeque.
    - Updating the indices cache, when the oldest message gets removed, involves iterating over the queue, so it has a time complexity of O(N), where N is the number of elements in the queue.
1. `fn delete(&mut self, id: &str) -> Option<Message>`:
    - Removing a message from the queue using an index has a time complexity of O(N) since it involves shifting elements after the removed index.
    - Updating the indices cache after deletion also has a time complexity of O(N) because it involves iterating over the remaining elements.
1. `fn get(&self, id: &str) -> Option<Message>`:
    - Retrieving a message based on the ID has a time complexity of O(1) for looking up the index in the hashmap and getting the corresponding element from the queue.
1. `fn get_all(&self) -> Vec<Message>`:
    - Cloning the entire queue for `get_all` has a time complexity of O(N), where N is the number of messages in the queue.

### Space Complexity:
-  The space complexity is O(N), where N is the number of messages in the queue.
The VecDeque and HashMap store the messages and index cache, respectively.

## Local development setup

### Rust and Cargo

Follow the [instruction](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install Rust and Cargo.

#### Cargo clippy linter

Follow the [instruction](https://github.com/rust-lang/rust-clippy#usage) to install `clippy`.

#### Grcov for test coverage
```sh
cargo install grcov
```

## Development

### Format code

```sh
cargo fmt
```

### Run clippy linter

```sh
cargo clippy  -- -D warnings
```

### Run tests
```sh
cargo test
```

### Prepare test coverage
```sh
RUSTFLAGS="-C instrument-coverage" cargo test
grcov . --binary-path target/debug/ -s . -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```

And then check the output (`./target/debug/coverage/`) in the browser (for HTML files).