use std::fmt::Debug;

mod concat;
pub use concat::Concat;

mod litteral;
pub use litteral::Litteral;

mod getter;
pub use getter::Getter;

pub trait Expression: Debug {}
