use super::{now, WAITS, WAITS_NUM};
use async_ach_waker::{WakerEntity, WakerPool, WakerToken};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

pub fn sleep(duration: Duration) -> Sleep {
    let deadline = now() + duration.as_nanos() as u64;
    Sleep {
        pool: &WAITS,
        deadline,
        token: None,
    }
}
pub fn sleep_until(deadline: u64) -> Sleep {
    Sleep {
        pool: &WAITS,
        deadline,
        token: None,
    }
}

pub struct Sleep {
    pool: &'static WakerPool<u64, WAITS_NUM>,
    deadline: u64,
    token: Option<WakerToken<'static, u64, WAITS_NUM>>,
}
impl Future for Sleep {
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
