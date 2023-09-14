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
mod spawner;
mod executor;
mod utils;


#[cfg(test)]
mod tests
{
    use crate::executor::Executor;

    use std::thread;
    use std::time::Duration;

    async fn async_function()
    {
        println!("Hello from async function!");
    }

    //--------------------------------------------------------------------------
    //  Tests the Executor.
    //--------------------------------------------------------------------------
    #[test]
    fn run_async_function()
    {
        let mut executor = Executor::new();
        let spawner = executor.spawner();
        spawner.spawn(async
        {
            for _ in 0..10
            {
                async_function().await;
                thread::sleep(Duration::from_millis(100));
            }
        });
        executor.run().unwrap();
    }
}
