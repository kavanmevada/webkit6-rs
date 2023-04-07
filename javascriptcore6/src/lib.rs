macro_rules! assert_initialized_main_thread {
    () => {};
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use ffi;

mod auto;
pub use crate::auto::*;
