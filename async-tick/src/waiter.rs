use async_ach_waker::{WakerEntity, WakerPool, WakerToken};
use atomic_polyfill::{AtomicU64, Ordering::SeqCst};
use core::ops::Deref;
use core::task::Waker;
use core::u64::MAX;

pub struct Waiter<const N: usize> {
    pool: WakerPool<u64, N>,
    next: AtomicU64,
}
impl<const N: usize> Waiter<N> {
    pub const fn new() -> Self {
        Self {
            pool: WakerPool::new(),
            next: AtomicU64::new(MAX),
        }
    }
    pub fn register(&self) -> Result<Token<N>, ()> {
        let token = self.pool.register()?;
        Ok(Token {
            next: &self.next,
            token,
        })
    }
    pub fn take_next(&self) -> Option<u64> {
        let next = self.next.swap(MAX, SeqCst);
        if next == MAX {
            None
        } else {
            Some(next)
        }
    }
    pub fn set_next(&self, next: u64) {
        self.next.fetch_min(next, SeqCst);
    }
}
impl<const N: usize> Deref for Waiter<N> {
    type Target = WakerPool<u64, N>;
    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

pub struct Token<'a, const N: usize> {
    next: &'a AtomicU64,
    token: WakerToken<'a, u64, N>,
}
impl<'a, const N: usize> Token<'a, N> {
    pub fn swap(&self, waker: Waker, deadline: u64) {
        self.next.fetch_min(deadline, SeqCst);
        self.token.swap(WakerEntity::new(waker, deadline));
    }
}
