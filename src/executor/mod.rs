/*

    Executor that schedules and executes tasks.

*/

mod single_thread_executor;
use single_thread_executor::SingleThreadExecutor;

use crate::utils::Result;
use crate::spawner::Spawner;

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
    //  Gets a Spawner for the Executor.
    //--------------------------------------------------------------------------
    pub fn spawner( &self ) -> Spawner
    {
        match self
        {
            Self::SingleThread(executor) =>
            {
                Spawner::new(executor.tasks.sender().clone())
            }
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
