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
/// use fexer::ExecutorBuilder;
/// use fexer_task::Task;
/// use std::sync::{ Arc, Mutex };
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
///    let mut executor = ExecutorBuilder::default().build();
///    executor.block_on(async
///    {
///        async_function().await;
///    });
/// }
/// ```
//------------------------------------------------------------------------------

mod builder;
mod executor;
mod worker;
mod sleep;

pub use builder::ExecutorBuilder;
pub use executor::Executor;


#[cfg(test)]
mod test
{
    use crate::sleep::SleepFuture;

    use crate::ExecutorBuilder;
    use fexer_task::Task;

    use std::sync::{ Arc, Mutex };
    use std::thread;
    use std::time::Duration;

    async fn async_function()
    {
        println!("Hello from async function!");
    }

    #[test]
    fn test_executor()
    {
        let executor = ExecutorBuilder::new()
            .num_threads(4)
            .build();

        let spawner = executor.sender();

        for _ in 0..10
        {
            let task = Task::new(async
            {
                println!("=========================================");
                println!("{}", thread::current().name().unwrap());
                SleepFuture::new(Duration::from_millis(1000)).await;
                async_function().await;
                println!("=========================================");
            });

            spawner.send(Arc::new(Mutex::new(task))).unwrap();
        }

        executor.run();
        thread::sleep(Duration::from_millis(2000));
    }

    #[test]
    fn test_executor_block_on()
    {
        let executor = ExecutorBuilder::new()
            .num_threads(4)
            .build();
        executor.block_on(async
        {
            async_function().await;
        });
        thread::sleep(Duration::from_millis(1000));
    }
}
