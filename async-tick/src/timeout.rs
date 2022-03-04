use super::sleep::*;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

pub fn timeout<T: Future + Unpin>(duration: Duration, future: T) -> Timeout<T> {
    let sleep = sleep(duration);
    Timeout { future, sleep }
}

pub struct Timeout<T: Future + Unpin> {
    future: T,
    sleep: Sleep,
}
impl<T: Future + Unpin> Future for Timeout<T> {
    type Output = Result<T::Output, ()>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // First, try polling the future
        if let Poll::Ready(v) = Pin::new(&mut self.future).poll(cx) {
            return Poll::Ready(Ok(v));
        }

        // Now check the timer
        match Pin::new(&mut self.sleep).poll(cx) {
            Poll::Ready(()) => Poll::Ready(Err(())),
            Poll::Pending => Poll::Pending,
        }
    }
}
