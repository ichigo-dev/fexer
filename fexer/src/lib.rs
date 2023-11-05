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
///    let spawner = executor.sender();
///    let task = Task::new(async_function());
///    spawner.send(Arc::new(Mutex::new(task)));
///    executor.run();
/// }
/// ```
//------------------------------------------------------------------------------

mod builder;
mod executor;
mod worker;

pub use builder::ExecutorBuilder;
pub use executor::Executor;

#[cfg(test)]
mod test
{
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

        for _ in 0..5
        {
            let task = Task::new(async
            {
                for _ in 0..5
                {
                    println!("=========================================");
                    println!("{}", thread::current().name().unwrap());
                    async_function().await;
                    println!("=========================================");

                    thread::sleep(Duration::from_millis(1000));
                }
            });

            spawner.send(Arc::new(Mutex::new(task))).unwrap();
        }

        executor.run();
        thread::sleep(Duration::from_millis(10000));
    }
}
