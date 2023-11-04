//------------------------------------------------------------------------------
/// # Executor
//------------------------------------------------------------------------------

use crate::worker::Worker;
use fexer_task::Task;
use fexer_channel::mpmc::{ channel, Sender };

use std::sync::Arc;

//------------------------------------------------------------------------------
/// Executor
//------------------------------------------------------------------------------
pub struct Executor
{
    workers: Vec<Worker>,
    sender: Sender<Arc<Task>>,
}

impl Executor
{
    //--------------------------------------------------------------------------
    /// Creates a new Executor.
    //--------------------------------------------------------------------------
    pub fn new( num_threads: usize ) -> Self
    {
        let (sender, receiver) = channel::<Arc<Task>>();
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
    pub fn sender( &self ) -> Sender<Arc<Task>>
    {
        self.sender.clone()
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
