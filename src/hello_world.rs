use futures::{Async, Future, Poll};

pub struct HelloWorld;

impl Future for HelloWorld {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready("HELLO WORLD".to_string()))
    }
}
