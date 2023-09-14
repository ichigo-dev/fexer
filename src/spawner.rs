use crate::task::Task;

use std::future::Future;
use std::rc::Rc;
use std::sync::mpsc::Sender;

pub struct Spawner
{
    sender: Sender<Rc<Task>>,
}

impl Spawner
{
    pub fn new( sender: Sender<Rc<Task>> ) -> Self
    {
        Self
        {
            sender,
        }
    }

    pub fn spawn<F>( &self, future: F )
        where F: Future<Output = ()> + Send + 'static
    {
        let task = Task::new(future);
        self.sender.send(Rc::new(task)).unwrap();
    }
}
