pub mod game_object;
pub mod player;
pub mod components;
pub mod anchor;
pub mod enemy;

pub use player::Player;
pub use components::Transform;
pub use anchor::Anchor;
pub use enemy::Enemy;
pub use game_object::GameObject;

use uid::Id;

// allows ObjectMarker and ObjectId
// to be compared for equality and be hashed (usable in HashMaps)
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct ObjectMarker; // Only used to mark that this Id<T> is for an object

pub type ObjectId = Id<ObjectMarker>;