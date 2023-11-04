//------------------------------------------------------------------------------
/// # Single Thread Executor
///
/// SingleThreadExecutor is a simple Executor that runs on a single thread. 
//------------------------------------------------------------------------------

use crate::task_queue::TaskQueue;
use crate::utils::Result;
use crate::waker::waker_fn;

use std::task::{ Context, Poll };

pub struct SingleThreadExecutor
{
    pub tasks: TaskQueue,
}

impl SingleThreadExecutor
{
    //--------------------------------------------------------------------------
    /// Creates a new Executor.
    //--------------------------------------------------------------------------
    pub fn new() -> Self
    {
        Self
        {
            tasks: TaskQueue::new(),
        }
    }

    //--------------------------------------------------------------------------
    /// Runs the Executor.
    //--------------------------------------------------------------------------
    pub fn run( &mut self ) -> Result<()>
    {
        loop
        {
            self.tasks.receive();
            while let Some(task) = self.tasks.pop()
            {
                let waker =
                {
                    let sender = self.tasks.sender();
                    let waker_task = task.clone();
                    waker_fn(move ||
                    {
                        sender.send(waker_task.clone()).unwrap();
                    })
                };
                let mut context = Context::from_waker(&waker);
                match task.poll(&mut context)
                {
                    Poll::Ready(_) => {},
                    Poll::Pending => {},
                }
            }
        }
    }
}
