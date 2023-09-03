/*

    Executor that schedules and executes tasks.

*/

use crate::task_queue::TaskQueue;
use crate::task::Task;
use crate::waker::waker_fn;
use crate::utils::Result;

use std::future::Future;
use std::task::{ Context, Poll };

pub struct Executor
{
    tasks: TaskQueue,
}

impl Executor
{
    //--------------------------------------------------------------------------
    //  Creates a new Executor.
    //--------------------------------------------------------------------------
    pub fn new() -> Self
    {
        Self
        {
            tasks: TaskQueue::new(),
        }
    }

    //--------------------------------------------------------------------------
    //  Spawns a new Task onto the queue.
    //--------------------------------------------------------------------------
    pub fn spawn<F>( &mut self, future: F )
        where F: Future<Output = ()> + Send + 'static
    {
        let task = Task::new(future);
        self.tasks.push(task);
    }

    //--------------------------------------------------------------------------
    //  Runs the Executor.
    //--------------------------------------------------------------------------
    pub fn run( &mut self ) -> Result<()>
    {
        loop
        {
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
                match task.future.borrow_mut().as_mut().poll(&mut context)
                {
                    Poll::Ready(_) => {},
                    Poll::Pending => {},
                }
            }
        }
    }
}
