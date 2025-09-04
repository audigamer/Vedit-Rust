pub mod player;
pub mod components;
pub mod anchor;

pub use player::Player;

use uid::Id;

#[derive(Clone, Copy, Eq, Hash, PartialEq)] // allows ObjectMarker and ObjectId
// to be compared for equality and be hashed (usable in HashMaps)
pub struct ObjectMarker; // Only used to mark that this Id<T> is for an object
pub type ObjectId = Id<ObjectMarker>;