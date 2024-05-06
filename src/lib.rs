#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::direction::{Direction, ALL_DIRECTIONS};
pub use self::directions_possible::DirectionsPossible;
pub use self::get_next_point::get_next_point;
pub use self::point::Point;

mod direction;
mod directions_possible;
mod get_next_point;
mod point;
