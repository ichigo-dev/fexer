/*

    # Fexer

    This is a runtime crate for executing tasks asynchronously and
    multithreaded.

*/

#![allow(dead_code)]

mod future;
mod task;
mod task_queue;
mod waker;
mod executor;
mod utils;
