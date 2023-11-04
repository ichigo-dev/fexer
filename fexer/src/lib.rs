#![allow(dead_code)]

//------------------------------------------------------------------------------
/// # Fexer
///
/// This is a runtime crate for executing tasks asynchronously and
/// multithreaded.
///
///
/// ## Example
///
/// ```
/// use fexer::Executor;
///
/// use std::thread;
///
/// async fn async_function()
/// {
///    println!("Hello from async function!");
/// }
///
/// fn main()
/// {
///    let mut executor = Executor::single();
///    let spawner = executor.spawner();
///    spawner.spawn(async_function);
///    executor.run().unwrap();
/// }
/// ```
//------------------------------------------------------------------------------

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
    // Tests the Executor.
    //--------------------------------------------------------------------------
    #[test]
    fn run_async_function()
    {
        let mut executor = Executor::single();
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
