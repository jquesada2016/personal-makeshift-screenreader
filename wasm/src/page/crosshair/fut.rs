use std::{
    cell::OnceCell,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

#[derive(Default)]
struct RequestAnimationFrameFut {
    state: Option<gloo::render::AnimationFrame>,
    is_done: Rc<OnceCell<()>>,
}

impl Future for RequestAnimationFrameFut {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state.is_none() {
            let waker = cx.waker().to_owned();
            let is_done = self.is_done.clone();

            self.state = Some(gloo::render::request_animation_frame(move |_| {
                let _ = is_done.set(());
                waker.wake();
            }));
        }

        if self.is_done.get().is_some() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub fn request_animation_frame() -> impl Future<Output = ()> {
    RequestAnimationFrameFut::default()
}
