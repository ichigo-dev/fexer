//------------------------------------------------------------------------------
/// # Executor
//------------------------------------------------------------------------------

use crate::worker::Worker;
use fexer_task::Task;
use fexer_channel::mpmc::{ channel, Sender };
use fexer_waker::waker_fn;

use std::sync::{ Arc, Mutex };
use std::task::{ Context, Poll };

//------------------------------------------------------------------------------
/// Executor
//------------------------------------------------------------------------------
pub struct Executor
{
    workers: Vec<Worker>,
    sender: Sender<Arc<Mutex<Task>>>,
}

impl Executor
{
    //--------------------------------------------------------------------------
    /// Creates a new Executor.
    //--------------------------------------------------------------------------
    pub fn new( num_threads: usize ) -> Self
    {
        let (sender, receiver) = channel::<Arc<Mutex<Task>>>();
        let mut workers = Vec::with_capacity(num_threads);
        for id in 0..num_threads
        {
            workers.push(Worker::new(id + 1, sender.clone(), receiver.clone()));
        }

        Self
        {
            workers,
            sender,
        }
    }

    //--------------------------------------------------------------------------
    /// Returns the sender.
    //--------------------------------------------------------------------------
    pub fn sender( &self ) -> Sender<Arc<Mutex<Task>>>
    {
        self.sender.clone()
    }

    //--------------------------------------------------------------------------
    /// Blocks the current thread on the given future.
    //--------------------------------------------------------------------------
    pub fn block_on<F>( &self, future: F ) -> F::Output
        where
            F: std::future::Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Mutex::new(Task::new(future)));
        let waker = waker_fn(|| {});
        let mut context = Context::from_waker(&waker);
        loop
        {
            if let Poll::Ready(output) = task.lock().unwrap().poll(&mut context)
            {
                return output;
            }
        }
    }

    //--------------------------------------------------------------------------
    /// Runs the Executor.
    //--------------------------------------------------------------------------
    pub fn run( &self )
    {
        for worker in &self.workers
        {
            worker.run();
        }
    }
}
