mod exercise;
mod sets;
mod training;
mod user;
mod workout;

pub use exercise::*;
use serde::{Deserialize, Serialize};
pub use sets::*;
use std::ops::Deref;
pub use training::*;
pub use user::*;
pub use workout::*;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialOrd,
    Default,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
    Ord,
)]
#[repr(transparent)]
pub struct Id(pub i64);

impl From<i64> for Id {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Deref for Id {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
