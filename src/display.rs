use futures::{Async, Future, Poll};
use std::fmt;

pub struct Display<T>(pub T);

impl<T> Future for Display<T>
where
    T: Future,
    T::Item: fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), T::Error> {
        let value: <T as futures::Future>::Item = try_ready!(self.0.poll());
        println!("{}", value);
        Ok(Async::Ready(()))
    }
}
