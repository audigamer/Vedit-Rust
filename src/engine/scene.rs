use std::{collections::HashMap};
use uid::Id;
use vector2::Vector2;

use crate::objects::{anchor::Anchor, components::*, enemy::Enemy, game_object::GameObject, player::Player, ObjectId};

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
    pub enemies: HashMap<ObjectId, Enemy>,
    pub local_transforms: HashMap<ObjectId, Transform>,
    pub global_transforms: HashMap<ObjectId, Transform>,
    pub hierarchies: HashMap<ObjectId, Hierarchy>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            anchors: HashMap::new(),
            enemies: HashMap::new(),
            local_transforms: HashMap::new(),
            global_transforms: HashMap::new(),
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
        let transform = self.local_transforms.get(&object_id).unwrap();
        let hierarchy = self.hierarchies.get(&object_id).unwrap();

        let Some(parent_id) = hierarchy.parent else {
            return
        };
        let parent_transform = self.global_transforms.get(&parent_id).unwrap();
        
        self.global_transforms.insert(object_id, transform.apply_transform(*parent_transform));
        
        // This works but it feels a little hacky
        // (I only did clone() because the compiler suggested that)
        self.update_child_transforms(object_id);
    }

    pub fn update_child_transforms(&mut self, object_id: ObjectId) {
        let hierarchy = self.hierarchies.get(&object_id).unwrap();

        for child_id in hierarchy.children.clone() {
            self.apply_parent_transform(child_id);
        }
    }

    fn add_object<T: GameObject>(
        map: &mut HashMap<ObjectId, T>,
        position: Vector2,
        local_transforms: &mut HashMap<ObjectId, Transform>,
        global_transforms: &mut HashMap<ObjectId, Transform>,
        hierarchies: &mut HashMap<ObjectId, Hierarchy>
    ) -> ObjectId {
        let id: ObjectId = Id::new();
        let object: T = GameObject::new();
        let transform: Transform = Transform::at_position(position);
        let hierarchy: Hierarchy = Hierarchy::empty();

        map.insert(id, object);
        local_transforms.insert(id, transform);
        global_transforms.insert(id, transform);
        hierarchies.insert(id, hierarchy);
        id
    }

    pub fn add_player(&mut self, position: Vector2) -> ObjectId {
        Self::add_object::<Player>(
            &mut self.players, position, &mut self.local_transforms,
            &mut self.global_transforms, &mut self.hierarchies
        )
    }

    pub fn add_anchor(&mut self, position: Vector2) -> ObjectId {
        Self::add_object::<Anchor>(
            &mut self.anchors, position, &mut self.local_transforms,
            &mut self.global_transforms, &mut self.hierarchies
        )
    }

    pub fn add_enemy(&mut self, position: Vector2) -> ObjectId {
        Self::add_object::<Enemy>(
            &mut self.enemies, position, &mut self.local_transforms,
            &mut self.global_transforms, &mut self.hierarchies
        )
    }
}