use std::future::Future;
use std::task::{Context, Poll,Wake};
use std::thread::{self, Thread};
use std::sync::{Arc};

struct Data {
    pending: bool,
}
impl Future for Data {
    type Output = i32;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
		println!("enter in poll");
        if self.pending {
            self.get_mut().pending = false;
			let waker = cx.waker().clone();
			std::thread::spawn(move ||{
				std::thread::sleep(std::time::Duration::from_secs(1));
				waker.wake();
			});
            Poll::Pending
        } else {
            Poll::Ready(10)
        }
    }
}
async fn show() -> i32 {
    let d = Data { pending: true };
	async {
		println!("before");
		let t = d.await;
		println!("end");
		t
	}.await
}
struct ThreadWaker(Thread);
impl Wake for ThreadWaker{
    fn wake(self: std::sync::Arc<Self>) {
		self.0.unpark();
    }
}
fn block_on<T>(fut: impl Future<Output = T>) -> T {
    // Pin the future so it can be polled.
    let mut fut = Box::pin(fut);

    // Create a new context to be passed to the future.
    let t = thread::current();
    let waker = Arc::new(ThreadWaker(t)).into();
    let mut cx = Context::from_waker(&waker);

    // Run the future to completion.
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(res) => return res,
            Poll::Pending => thread::park(),
        }
    }
}
fn main() {
    let future = show();
	let t = block_on(future);
	println!("end, {}",t);
}
