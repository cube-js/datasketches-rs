//! `dsrs` contains bindings for a subset of [Apache DataSketches](https://github.com/apache/datasketches-cpp).

mod bridge;
pub mod counters;
mod error;
pub mod stream_reducer;
mod wrapper;

pub use error::DataSketchesError;
pub use wrapper::*;
