//------------------------------------------------------------------------------
/// # MPMC channel
//------------------------------------------------------------------------------
mod error;
mod sender;
mod receiver;

pub use error::MpmcError;
pub use sender::Sender;
pub use receiver::Receiver;

use std::sync::mpsc;


//------------------------------------------------------------------------------
/// Creates a new MPMC channel.
//------------------------------------------------------------------------------
pub fn channel<T>() -> (Sender<T>, Receiver<T>)
{
    let (sender, receiver) = mpsc::channel();
    let sender = Sender::new(sender);
    let receiver = Receiver::new(receiver);
    (sender, receiver)
}
