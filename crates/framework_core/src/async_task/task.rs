use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[must_use = "tasks are cancelled when dropped, use `.detach()` to run the task in the background"]
pub struct AsyncTask<T> {
    task: async_task::Task<T>,
}

impl<T> AsyncTask<T> {
    pub fn is_finished(&self) -> bool {
        self.task.is_finished()
    }

    /// Cancels the task and returns the result if it's already finished
    pub fn join(self) -> Option<T> {
        if self.is_finished() {
            Some(crate::async_task::block_on(self.task.cancel()).unwrap())
        } else {
            None
        }
    }

    /// Run the task without requiring the result to be used
    pub fn detach(self) {
        self.task.detach()
    }
}

impl<T> Future for AsyncTask<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Future::poll(Pin::new(&mut self.task), cx)
    }
}

impl<T> From<async_task::Task<T>> for AsyncTask<T> {
    fn from(task: async_task::Task<T>) -> Self {
        Self { task }
    }
}
