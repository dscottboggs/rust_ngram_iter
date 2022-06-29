#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
mod iter;
mod iterable;
mod state;

pub use iter::Iter;
pub use iterable::{Iterable, WORD_JOINER};
pub use state::State;
