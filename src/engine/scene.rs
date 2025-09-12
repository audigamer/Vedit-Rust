use std::collections::{HashMap, HashSet};
use uid::Id;
use vector2::Vector2;

use crate::objects::*;
/*
Scenes are containers for objects and components, stored in a HashMap with
ObjectId keys in ECS (entity component system) style. The components which
an object needs are stored with the same ObjectId as the object. For example,
to get the player's scene tree transform, you do:

let scene: Scene = ...
let player_id: ObjectId = ...
let hierarchy = scene.global_transforms.get(&player_id).unwrap();

Scenes have their own ID, because they can have their own components like any
other game object. When creating a Scene, a set for its direct children gets
added to the children HashMap.
*/
pub struct Scene {
    pub id: ObjectId,
    pub players: HashMap<ObjectId, Player>,
    pub anchors: HashMap<ObjectId, Anchor>,
    pub enemies: HashMap<ObjectId, Enemy>,
    pub local_transforms: HashMap<ObjectId, Transform>,
    pub global_transforms: HashMap<ObjectId, Transform>,

    // Game object relationships
    pub parents: HashMap<ObjectId, ObjectId>,
    pub children: HashMap<ObjectId, HashSet<ObjectId>>,
    //pub hierarchies: HashMap<ObjectId, Hierarchy>,
}

impl Scene {
    pub fn new() -> Self {
        let scene_id: ObjectId = Id::new();
        // Add scene's children set
        let children: HashMap<ObjectId, HashSet<ObjectId>> = 
            HashMap::from([(scene_id, HashSet::new())]);

        Self {
            id: scene_id,
            players: HashMap::new(),
            anchors: HashMap::new(),
            enemies: HashMap::new(),
            local_transforms: HashMap::new(),
            global_transforms: HashMap::new(),
            parents: HashMap::new(),
            children
        }
    }

    // Adds the child ID to the parent ID's children list and removes it from
    // the old parent's children list
    pub fn append_child(&mut self, parent_id: ObjectId, child_id: ObjectId) {
        self.children.get_mut(&parent_id).unwrap().insert(child_id);
        
        let old_parent_option: Option<ObjectId> = self.parents.insert(child_id, parent_id);

        if let Some(old_parent_id) = old_parent_option {
            //let old_parent: &mut Hierarchy = self.hierarchies.get_mut(&old_parent_id).unwrap();
            self.children.get_mut(&old_parent_id).unwrap().remove(&child_id);
        }
    }

    // Should be ran after each object's initial state (local transforms) gets loaded
    // to initialize the global transforms.
    pub fn initialize_transforms(&mut self) {
        let children: HashSet<ObjectId> = self.children.get(&self.id).unwrap().clone();
        let default_transform: Transform = Transform::at_position(Vector2::ZERO);
        self.recalculate_child_transforms(children, &default_transform);
    }

    fn recalculate_child_transforms(&mut self, children: HashSet<ObjectId>, parent_transform: &Transform) {
        println!("{}", children.len());
        for child_id in children {
            let transform = self.local_transforms.get(&child_id).unwrap();
            let new_transform = transform.apply_transform(*parent_transform);
            self.global_transforms.insert(child_id, new_transform);
            
            let next_children: HashSet<ObjectId> = self.children.get(&child_id).unwrap().clone();
            self.recalculate_child_transforms(next_children, &new_transform);
        }
        
    }

    // Helper function to remove repetition from the other add_<object> functions
    fn add_object<T: GameObject>(
        map: &mut HashMap<ObjectId, T>,
        position: Vector2,
        local_transforms: &mut HashMap<ObjectId, Transform>,
        global_transforms: &mut HashMap<ObjectId, Transform>,
        parents: &mut HashMap<ObjectId, ObjectId>,
        children: &mut HashMap<ObjectId, HashSet<ObjectId>>,
        scene_id: &ObjectId
    ) -> ObjectId {
        let id: ObjectId = Id::new();
        let object: T = GameObject::new();
        let transform: Transform = Transform::at_position(position);

        map.insert(id, object);
        local_transforms.insert(id, transform);
        global_transforms.insert(id, transform);
        parents.insert(id, *scene_id);
        children.insert(id, HashSet::new());
        children.get_mut(&scene_id).unwrap().insert(id);
        id
    }

    pub fn add_player(&mut self, position: Vector2) -> ObjectId {
        Self::add_object::<Player>(
            &mut self.players, position, &mut self.local_transforms,
            &mut self.global_transforms, &mut self.parents, &mut self.children, &self.id
        )
    }

    pub fn add_anchor(&mut self, position: Vector2) -> ObjectId {
        Self::add_object::<Anchor>(
            &mut self.anchors, position, &mut self.local_transforms,
            &mut self.global_transforms, &mut self.parents, &mut self.children, &self.id
        )
    }

    pub fn add_enemy(&mut self, position: Vector2) -> ObjectId {
        Self::add_object::<Enemy>(
            &mut self.enemies, position, &mut self.local_transforms,
            &mut self.global_transforms, &mut self.parents, &mut self.children, &self.id
        )
    }
}