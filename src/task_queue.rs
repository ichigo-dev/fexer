//------------------------------------------------------------------------------
/// # Task Queue
///
/// Queue of Tasks to be executed by the executor.
//------------------------------------------------------------------------------

use crate::task::Task;

use std::rc::Rc;
use std::sync::mpsc::{ self, Receiver, Sender };

//------------------------------------------------------------------------------
/// TaskQueue
//------------------------------------------------------------------------------
pub struct TaskQueue
{
    sender: Sender<Rc<Task>>,
    receiver: Receiver<Rc<Task>>,
    tasks: Vec<Rc<Task>>,
}

impl TaskQueue
{
    //--------------------------------------------------------------------------
    /// Creates a new TaskQueue.
    //--------------------------------------------------------------------------
    pub fn new() -> Self
    {
        let (sender, receiver) = mpsc::channel();
        Self
        {
            sender,
            receiver,
            tasks: Vec::new(),
        }
    }

    //--------------------------------------------------------------------------
    //  Returns the sender of the TaskQueue.
    //--------------------------------------------------------------------------
    pub fn sender( &self ) -> Sender<Rc<Task>>
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
    //  Pushes a new Task onto the queue.
    //--------------------------------------------------------------------------
    pub fn push( &mut self, task: Task )
    {
        self.tasks.push(Rc::new(task));
    }

    //--------------------------------------------------------------------------
    //  Pops a Task off the queue.
    //--------------------------------------------------------------------------
    pub fn pop(&mut self) -> Option<Rc<Task>>
    {
        self.tasks.pop()
    }

    //--------------------------------------------------------------------------
    //  Returns true if the queue is empty.
    //--------------------------------------------------------------------------
    pub fn is_empty(&self) -> bool
    {
        self.tasks.is_empty()
    }
}
