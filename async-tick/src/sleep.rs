use super::{now, WAITS, WAITS_NUM};
use async_ach_waker::{WakerEntity, WakerPool, WakerToken};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

pub fn sleep(duration: Duration) -> Sleep<'static, WAITS_NUM> {
    let deadline = now() + duration.as_nanos() as u64;
    Sleep {
        pool: &WAITS,
        deadline,
        token: None,
    }
}
pub fn sleep_until(deadline: u64) -> Sleep<'static, WAITS_NUM> {
    Sleep {
        pool: &WAITS,
        deadline,
        token: None,
    }
}

pub struct Sleep<'a, const N: usize> {
    pool: &'a WakerPool<u64, N>,
    deadline: u64,
    token: Option<WakerToken<'a, u64, N>>,
}
impl<'a, const N: usize> Future for Sleep<'a, N> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let deadline = self.deadline;
        if deadline < now() {
            return Poll::Ready(());
        }
        let waker = cx.waker();
        if let Some(token) = &self.token {
            token.swap(WakerEntity::new(waker.clone(), deadline));
        } else if let Ok(token) = self.pool.register() {
            token.swap(WakerEntity::new(waker.clone(), deadline));
            self.token = Some(token);
        } else {
            waker.wake_by_ref();
        }
        if deadline < now() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
