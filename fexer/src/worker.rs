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
        let _ = thread::Builder::new().name(self.id.to_string()).spawn(move ||
        {
            loop
            {
                let task = match receiver.recv()
                {
                    Ok(task) => task,
                    Err(_) => break,
                };
                let cloned_task = task.clone();
                let waker =
                {
                    let sender = sender.clone();
                    waker_fn(move ||
                    {
                        let _ = sender.send(cloned_task.clone());
                    })
                };
                let mut context = Context::from_waker(&waker);

                let mut guard = match task.lock()
                {
                    Ok(guard) => guard,
                    Err(_) => break,
                };
                match guard.poll(&mut context)
                {
                    Poll::Ready(_) => {},
                    Poll::Pending => {},
                };
            }
        });
    }
}
