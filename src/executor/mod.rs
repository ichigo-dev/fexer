//------------------------------------------------------------------------------
/// # Executor
///
/// Executor that schedules and executes tasks.
//------------------------------------------------------------------------------

mod single_thread_executor;
mod multi_thread_executor;
use single_thread_executor::SingleThreadExecutor;
use multi_thread_executor::MultiThreadExecutor;

use crate::utils::Result;
use crate::spawner::Spawner;

//------------------------------------------------------------------------------
/// Executor
//------------------------------------------------------------------------------
pub enum Executor
{
    SingleThread(SingleThreadExecutor),
    MultiThread(MultiThreadExecutor),
}

impl Executor
{
    //--------------------------------------------------------------------------
    /// Creates a new Executor.
    //--------------------------------------------------------------------------
    pub fn single() -> Self
    {
        Self::SingleThread(SingleThreadExecutor::new())
    }

    pub fn multi() -> Self
    {
        Self::MultiThread(MultiThreadExecutor::new())
    }

    //--------------------------------------------------------------------------
    /// Gets a Spawner for the Executor.
    //--------------------------------------------------------------------------
    pub fn spawner( &self ) -> Spawner
    {
        match self
        {
            Self::SingleThread(executor) =>
            {
                Spawner::new(executor.tasks.sender().clone())
            },
            Self::MultiThread(_executor) =>
            {
                unimplemented!();
                //Spawner::new(executor.spawner())
            },
        }
    }

    //--------------------------------------------------------------------------
    /// Runs the Executor.
    //--------------------------------------------------------------------------
    pub fn run( &mut self ) -> Result<()>
    {
        match self
        {
            Self::SingleThread(executor) => executor.run(),
            Self::MultiThread(executor) => executor.run(),
        }
    }
}
