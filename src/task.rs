/*

    Tasks executed asynchronously by the runtime. This is an abstraction
    structure for coroutines.

*/

use crate::future::BoxedFuture;

use std::cell::RefCell;
use std::future::Future;

pub struct Task
{
    pub future: RefCell<BoxedFuture<'static, ()>>,
}

impl Task
{
    //--------------------------------------------------------------------------
    //  Creates a new Task.
    //--------------------------------------------------------------------------
    pub fn new<F>( future: F ) -> Self
        where F: Future<Output = ()> + Send + 'static
    {
        Self
        {
            future: RefCell::new(Box::pin(future)),
        }
    }
}
