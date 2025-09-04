use std::collections::HashMap;
use uid::Id;
use vector2::Vector2;

use crate::objects::{anchor::Anchor, components::*, player::Player, ObjectId};

/*
Scenes are containers for objects and components, stored in a HashMap with
ObjectId keys in ECS (entity component system) style. The components which
an object needs are stored with the same ObjectId as the object. For example,
to get the player's scene tree hierarchy, you do:

let player_id: ObjectId = ...
let hierarchy = scene.hierarchies.get(&player_id).unwrap();
*/
pub struct Scene {
    pub players: HashMap<ObjectId, Player>,
    pub anchors: HashMap<ObjectId, Anchor>,
    pub transforms: HashMap<ObjectId, Transform>,
    pub hierarchies: HashMap<ObjectId, Hierarchy>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            anchors: HashMap::new(),
            transforms: HashMap::new(),
            hierarchies: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, position: Vector2) -> ObjectId {
        let player_id: ObjectId = Id::new();
        let player = Player::new(position);
        //let transform: Transform = Transform::at_position(position);
        let hierarchy = Hierarchy { parent: None, children: vec![] };

        self.players.insert(player_id, player);
        //self.transforms.insert()
        self.hierarchies.insert(player_id, hierarchy);
        player_id
    }

}