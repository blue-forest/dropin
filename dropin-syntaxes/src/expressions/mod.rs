use std::fmt::Debug;

mod concat;
pub use concat::Concat;

mod getter;
pub use getter::Getter;

mod litteral;
pub use litteral::Litteral;

mod quantifier;
pub use quantifier::Quantifier;

pub trait Expression: Debug {}
