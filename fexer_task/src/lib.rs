//------------------------------------------------------------------------------
/// # Fexer Task
///
/// Tasks executed asynchronously by the runtime. This is an abstraction
/// structure for coroutines.
//------------------------------------------------------------------------------

use std::future::Future;
use std::task::{ Context, Poll };
use std::pin::Pin;

pub type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

//------------------------------------------------------------------------------
/// Task
//------------------------------------------------------------------------------
pub struct Task
{
    pub future: BoxedFuture<()>,
}

impl Task
{
    //--------------------------------------------------------------------------
    /// Creates a new Task.
    //--------------------------------------------------------------------------
    pub fn new<F>( future: F ) -> Self
        where F: Future<Output = ()> + Send + 'static
    {
        Self
        {
            future: Box::pin(future),
        }
    }

    //--------------------------------------------------------------------------
    /// Polls the task.
    //--------------------------------------------------------------------------
    pub fn poll( &mut self, context: &mut Context ) -> Poll<()>
    {
        self.future.as_mut().poll(context)
    }
}
