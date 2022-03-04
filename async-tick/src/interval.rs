use super::{now, WAITS, WAITS_NUM};
use async_ach_waker::{WakerEntity, WakerPool, WakerToken};
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;
use futures_util::Stream;

pub fn interval(period: Duration) -> Interval {
    Interval {
        pool: &WAITS,
        last: now(),
        period,
        token: None,
    }
}

pub struct Interval {
    pool: &'static WakerPool<u64, WAITS_NUM>,
    last: u64,
    period: Duration,
    token: Option<WakerToken<'static, u64, WAITS_NUM>>,
}
impl Stream for Interval {
    type Item = ();
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let next_time = self.last + self.period.as_nanos() as u64;
        if next_time < now() {
            self.last = next_time;
            return Poll::Ready(Some(()));
        }
        let waker = cx.waker();
        if let Some(token) = &self.token {
            token.swap(WakerEntity::new(waker.clone(), next_time));
        } else if let Ok(token) = self.pool.register() {
            token.swap(WakerEntity::new(waker.clone(), next_time));
            self.token = Some(token);
        } else {
            waker.wake_by_ref();
        }
        if next_time < now() {
            self.last = next_time;
            Poll::Ready(Some(()))
        } else {
            Poll::Pending
        }
    }
}
