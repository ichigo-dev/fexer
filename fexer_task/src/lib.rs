//------------------------------------------------------------------------------
/// # Task
///
/// Tasks executed asynchronously by the runtime. This is an abstraction
/// structure for coroutines.
//------------------------------------------------------------------------------

use std::cell::RefCell;
use std::future::Future;
use std::task::{ Context, Poll };
use std::pin::Pin;

pub type BoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

//------------------------------------------------------------------------------
/// Task
//------------------------------------------------------------------------------
pub struct Task
{
    pub future: RefCell<BoxedFuture<'static, ()>>,
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
            future: RefCell::new(Box::pin(future)),
        }
    }

    //--------------------------------------------------------------------------
    /// Polls the task.
    //--------------------------------------------------------------------------
    pub fn poll( &self, context: &mut Context ) -> Poll<()>
    {
        self.future.borrow_mut().as_mut().poll(context)
    }
}
