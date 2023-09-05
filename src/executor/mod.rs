/*

    Executor that schedules and executes tasks.

*/

mod single_thread_executor;

use single_thread_executor::SingleThreadExecutor;

use crate::utils::Result;

use std::future::Future;

pub enum Executor
{
    SingleThread(SingleThreadExecutor),
}

impl Executor
{
    //--------------------------------------------------------------------------
    //  Creates a new Executor.
    //--------------------------------------------------------------------------
    pub fn new() -> Self
    {
        Self::SingleThread(SingleThreadExecutor::new())
    }

    //--------------------------------------------------------------------------
    //  Spawns a new Task onto the queue.
    //--------------------------------------------------------------------------
    pub fn spawn<F>( &mut self, future: F )
        where F: Future<Output = ()> + Send + 'static
    {
        match self
        {
            Self::SingleThread(executor) => executor.spawn(future),
        }
    }

    //--------------------------------------------------------------------------
    //  Runs the Executor.
    //--------------------------------------------------------------------------
    pub fn run( &mut self ) -> Result<()>
    {
        match self
        {
            Self::SingleThread(executor) => executor.run(),
        }
    }
}
