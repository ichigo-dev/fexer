use crate::task::Task;
use crate::waker::waker_fn;
use crate::utils::Result;

use std::task::{ Context, Poll };
use std::rc::Rc;

use crossbeam_channel::{ Sender, Receiver };

pub struct MultiThreadExecutor
{
    sender: Sender<Rc<Task>>,
    receiver: Receiver<Rc<Task>>,
    pub tasks: Vec<Rc<Task>>,
}

impl MultiThreadExecutor
{
    //--------------------------------------------------------------------------
    //  Creates a new Executor.
    //--------------------------------------------------------------------------
    pub fn new() -> Self
    {
        let (sender, receiver) = crossbeam_channel::unbounded();
        Self
        {
            sender,
            receiver,
            tasks: Vec::new(),
        }
    }

    //--------------------------------------------------------------------------
    //  Returns the sender of the tasks.
    //--------------------------------------------------------------------------
    pub fn spawner( &self ) -> Sender<Rc<Task>>
    {
        self.sender.clone()
    }

    //--------------------------------------------------------------------------
    //  Receives Tasks from the sender.
    //--------------------------------------------------------------------------
    pub fn receive( &mut self )
    {
        for task in self.receiver.try_iter()
        {
            self.tasks.push(task);
        }
    }

    //--------------------------------------------------------------------------
    //  Runs the Executor.
    //--------------------------------------------------------------------------
    pub fn run( &mut self ) -> Result<()>
    {
        loop
        {
            self.receive();
            while let Some(task) = self.tasks.pop()
            {
                let waker =
                {
                    let sender = self.sender.clone();
                    let waker_task = task.clone();
                    waker_fn(move ||
                    {
                        sender.send(waker_task.clone()).unwrap();
                    })
                };
                let mut context = Context::from_waker(&waker);
                match task.future.borrow_mut().as_mut().poll(&mut context)
                {
                    Poll::Ready(_) => {},
                    Poll::Pending => {},
                }
            }
        }
    }
}
