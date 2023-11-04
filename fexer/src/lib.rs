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

mod builder;
mod executor;
mod worker;

pub use builder::ExecutorBuilder;
pub use executor::Executor;
