use alloc::boxed::Box;
use core::task::{Context, Poll};
use core::{future::Future, pin::Pin};

pub mod executor;
pub mod keyboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(usize);

pub struct Task {
  future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
  pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
    Task {
      future: Box::pin(future),
    }
  }

  fn id(&self) -> TaskId {
    use core::ops::Deref;

    let addr = Pin::deref(&self.future) as *const _ as *const () as usize;
    TaskId(addr)
  }
}

impl Task {
  fn poll(&mut self, context: &mut Context) -> Poll<()> {
    self.future.as_mut().poll(context)
  }
}
