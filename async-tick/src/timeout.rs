use super::sleep::*;
use super::WAITS_NUM;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

pub fn timeout<T: Future + Unpin>(duration: Duration, future: T) -> Timeout<'static, T, WAITS_NUM> {
    let sleep = sleep(duration);
    Timeout { future, sleep }
}

pub struct Timeout<'a, T: Future + Unpin, const N: usize> {
    future: T,
    sleep: Sleep<'a, N>,
}
impl<'a, T: Future + Unpin, const N: usize> Future for Timeout<'a, T, N> {
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
