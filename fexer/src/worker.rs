//------------------------------------------------------------------------------
/// # Worker
//------------------------------------------------------------------------------

use fexer_channel::mpmc::{ Sender, Receiver };
use fexer_task::Task;
use fexer_waker::waker_fn;

use std::thread;
use std::task::{ Context, Poll };
use std::sync::{ Arc, Mutex };

//------------------------------------------------------------------------------
/// Worker
//------------------------------------------------------------------------------
pub(crate) struct Worker
{
    id: usize,
    sender: Sender<Arc<Mutex<Task>>>,
    receiver: Receiver<Arc<Mutex<Task>>>,
}

impl Worker
{
    //--------------------------------------------------------------------------
    /// Creates a new Worker.
    //--------------------------------------------------------------------------
    pub(crate) fn new
    (
        id: usize,
        sender: Sender<Arc<Mutex<Task>>>,
        receiver: Receiver<Arc<Mutex<Task>>>,
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
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        thread::spawn(move ||
        {
            loop
            {
                let task = receiver.recv().unwrap();
                let waker =
                {
                    let sender = sender.clone();
                    waker_fn(move ||
                    {
                        sender.send(task.clone()).unwrap()
                    })
                };
                let mut context = Context::from_waker(&waker);
                match task.lock().unwrap().poll(&mut context)
                {
                    Poll::Ready(_) => {},
                    Poll::Pending => {},
                };
            }
        });
    }
}
