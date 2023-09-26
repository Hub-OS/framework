use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};

struct PromisedFuture<T> {
    mutex: Arc<Mutex<(Option<T>, Option<Waker>)>>,
}

impl<T> Future for PromisedFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.mutex.lock().unwrap();

        match guard.0.take() {
            Some(value) => Poll::Ready(value),
            None => {
                let waker = cx.waker().clone();
                guard.1 = Some(waker);
                Poll::Pending
            }
        }
    }
}

pub fn promise_future<T>() -> (impl FnOnce(T), impl Future<Output = T>) {
    let mutex = Arc::new(Mutex::new((None, None)));
    let promised_future = PromisedFuture {
        mutex: mutex.clone(),
    };

    let resolve_future = move |value| {
        let mut guard = mutex.lock().unwrap();

        guard.0 = Some(value);

        if let Some(waker) = guard.1.take() {
            waker.wake();
        }
    };

    (resolve_future, promised_future)
}
