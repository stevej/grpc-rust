mod task_data;

mod stream_single;
mod stream_merge2;
mod stream_with_eof;
mod stream_with_eof_and_error;
pub mod stream_deferred;

pub use self::task_data::TaskRcMut;

pub use self::stream_single::stream_single;

pub use self::stream_merge2::stream_merge2;
pub use self::stream_merge2::StreamMerge2;
pub use self::stream_merge2::Merged2Item;

pub use self::stream_with_eof::*;
pub use self::stream_with_eof_and_error::*;

pub use self::stream_deferred::stream_deferred;
