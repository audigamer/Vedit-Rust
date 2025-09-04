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

    pub fn append_child(&mut self, parent_id: ObjectId, child_id: ObjectId) {
        let parent = self.hierarchies.get_mut(&parent_id).unwrap();
        parent.children.push(child_id);

        let child = self.hierarchies.get_mut(&child_id).unwrap();
        child.parent = Some(parent_id);

        self.apply_parent_transform(child_id);
    }

    pub fn apply_parent_transform(&mut self, object_id: ObjectId) {
        let transform = self.transforms.get(&object_id).unwrap();
        let hierarchy = self.hierarchies.get(&object_id).unwrap();

        let Some(parent_id) = hierarchy.parent else {
            return
        };
        let parent_transform = self.transforms.get(&parent_id).unwrap();
        
        // TODO: Make the transforms HashMap only store the local transforms of the objects.
        // The global transform (with the parents' transforms applied) should be stored elsewhere.
        // I'm not sure how this should be tackled yet. 
        self.transforms.insert(object_id, transform.apply_transform(*parent_transform));
    }

    pub fn add_player(&mut self, position: Vector2) -> ObjectId {
        let player_id: ObjectId = Id::new();
        let player = Player::new();
        let transform: Transform = Transform::at_position(position);
        let hierarchy = Hierarchy { parent: None, children: vec![] };

        self.players.insert(player_id, player);
        self.transforms.insert(player_id, transform);
        self.hierarchies.insert(player_id, hierarchy);
        player_id
    }

    pub fn add_anchor(&mut self, position: Vector2) -> ObjectId {
        let anchor_id: ObjectId = Id::new();

        let anchor = Anchor::new();
        let transform = Transform::at_position(position);
        let hierarchy = Hierarchy { parent: None, children: vec![] };

        self.anchors.insert(anchor_id, anchor);
        self.transforms.insert(anchor_id, transform);
        self.hierarchies.insert(anchor_id, hierarchy);

        anchor_id
    }
}