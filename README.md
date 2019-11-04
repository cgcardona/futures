# Futures

This is a [Rust]() app demoing futures using [Tokio](https://tokio.rs).

## Usage

First, clone the repo

```
git clone https://github.com/cgcardona/futures.git
```

Next, change directories and build app and deps

```
cd futures
cd build
```

Run the app

```
cargo run
```

## The `Future` trait

To use futures you must implement the `Future` trait.

```rust
trait Future {
    /// The type of the value returned when the future completes.
    type Item;

    /// The type representing errors that occurred while processing the computation.
    type Error;

    /// The function that will be repeatedly called to see if the future
    /// has completed or not. The `Async` enum can either be `Ready` or
    /// `NotReady` and indicates whether the future is ready to produce
    /// a value or not.
    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error>;
}
```
