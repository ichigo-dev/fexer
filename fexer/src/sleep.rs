use std::time::{ Instant, Duration };
use std::future::Future;
use std::pin::Pin;
use std::task::{ Poll, Context };

pub struct SleepFuture
{
    time: Instant,
}

impl SleepFuture
{
    #[allow(dead_code)]
    pub fn new(duration: Duration) -> Self
    {
        SleepFuture
        {
            time: Instant::now() + duration,
        }
    }
}

impl Future for SleepFuture
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()>
    {
        let now = Instant::now();
        if now >= self.time
        {
            Poll::Ready(())
        }
        else
        {
            println!("Still sleeping...");
            let waker = cx.waker().clone();
            let time = self.time;

            std::thread::spawn(move ||
            {
                std::thread::sleep(time - now);
                waker.wake();
            });

            Poll::Pending
        }
    }
}
