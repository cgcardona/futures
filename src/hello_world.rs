use futures::{Async, Future, Poll};

#[derive(Debug)]
pub struct HelloWorld;

impl Future for HelloWorld {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready("HELLO WORLD".into()))
    }
}
