mod player;
mod components;
pub use player::Player;

use uid::Id;

pub struct ObjectMarker; // Only used to mark that this Id<T> is for a object
pub type ObjectId = Id<ObjectMarker>;