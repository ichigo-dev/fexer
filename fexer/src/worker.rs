//------------------------------------------------------------------------------
/// # Worker
//------------------------------------------------------------------------------

use fexer_channel::mpmc::{ Sender, Receiver };
use fexer_task::Task;
use fexer_waker::waker_fn;

use std::thread;
use std::task::{ Context, Poll };
use std::sync::Arc;

//------------------------------------------------------------------------------
/// Worker
//------------------------------------------------------------------------------
pub(crate) struct Worker
{
    id: usize,
    sender: Sender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}

impl Worker
{
    //--------------------------------------------------------------------------
    /// Creates a new Worker.
    //--------------------------------------------------------------------------
    pub(crate) fn new
    (
        id: usize,
        sender: Sender<Arc<Task>>,
        receiver: Receiver<Arc<Task>>,
    ) -> Self
    {
        Self
        {
            id,
            sender,
            receiver,
        }
    }

    //--------------------------------------------------------------------------
    /// Runs the Worker.
    //--------------------------------------------------------------------------
    pub(crate) fn run( &self )
    {
        let id = self.id;
        let receiver = self.receiver.clone();
        thread::spawn(move ||
        {
            loop
            {
                /*
                let task = receiver.recv().unwrap();
                let waker =
                {
                    let sender = self.sender.clone();
                    waker_fn(move || sender.send(task.clone()).unwrap())
                };
                let context = Context::from_waker(&waker);
                match task.poll(&mut context)
                {
                    Poll::Ready(_) => {},
                    Poll::Pending => {},
                }
                */
            }
        });
    }
}
