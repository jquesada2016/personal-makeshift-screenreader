use futures::Future;
use std::task::Poll;

#[derive(Default)]
pub struct RequestAnimationFrameFuture(bool);

impl Future for RequestAnimationFrameFuture {
  type Output = ();

  fn poll(
    mut self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    if self.0 {
      Poll::Ready(())
    } else {
      self.0 = true;
      let waker = cx.waker().clone();

      leptos::request_animation_frame(move || waker.wake());

      Poll::Pending
    }
  }
}

pub fn request_animation_frame() -> impl Future<Output = ()> {
  RequestAnimationFrameFuture::default()
}
