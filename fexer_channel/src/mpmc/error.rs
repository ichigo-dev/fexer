//------------------------------------------------------------------------------
/// # MPMC error
//------------------------------------------------------------------------------

use std::sync::{ mpsc, PoisonError };
use std::fmt::{ self, Debug, Display, Formatter };


//------------------------------------------------------------------------------
/// MpmcError
//------------------------------------------------------------------------------
pub enum MpmcError<T>
{
    SendError(mpsc::SendError<T>),
    RecvError(mpsc::RecvError),
    TryRecvError(mpsc::TryRecvError),
    PoisonError,
}

impl<T> Debug for MpmcError<T>
{
    fn fmt( &self, f: &mut Formatter<'_> ) -> fmt::Result
    {
        match self
        {
            Self::SendError(error) => write!(f, "SendError: {:?}", error),
            Self::RecvError(error) => write!(f, "RecvError: {:?}", error),
            Self::TryRecvError(error) => write!(f, "TryRecvError: {:?}", error),
            Self::PoisonError => write!(f, "PoisonError"),
        }
    }
}

impl<T> Display for MpmcError<T>
{
    fn fmt( &self, f: &mut Formatter<'_> ) -> fmt::Result
    {
        match self
        {
            Self::SendError(error) => write!(f, "SendError: {}", error),
            Self::RecvError(error) => write!(f, "RecvError: {}", error),
            Self::TryRecvError(error) => write!(f, "TryRecvError: {}", error),
            Self::PoisonError => write!(f, "PoisonError"),
        }
    }
}

impl<T> From<mpsc::SendError<T>> for MpmcError<T>
{
    fn from( error: mpsc::SendError<T> ) -> Self
    {
        Self::SendError(error)
    }
}

impl<T> From<mpsc::RecvError> for MpmcError<T>
{
    fn from( error: mpsc::RecvError ) -> Self
    {
        Self::RecvError(error)
    }
}

impl<T> From<mpsc::TryRecvError> for MpmcError<T>
{
    fn from( error: mpsc::TryRecvError ) -> Self
    {
        Self::TryRecvError(error)
    }
}

impl<T, E> From<PoisonError<E>> for MpmcError<T>
{
    fn from( _error: PoisonError<E> ) -> Self
    {
        Self::PoisonError
    }
}
