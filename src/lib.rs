/*

    # Fexer

    This is a runtime crate for executing tasks asynchronously and
    multithreaded.

*/

#![allow(dead_code)]

mod future;
mod task;
mod task_queue;
mod waker;
mod executor;
mod utils;


#[cfg(test)]
mod tests
{
    use crate::executor::Executor;
    use crate::utils::Result;

    use std::thread;
    use std::time::Duration;

    //--------------------------------------------------------------------------
    //  Tests the Executor.
    //--------------------------------------------------------------------------
    #[test]
    fn run_async_function()
    {
        let mut executor = Executor::new();
        executor.spawn(async
        {
            println!("Hello from async function!");
        });
        executor.run().unwrap();
    }
}
