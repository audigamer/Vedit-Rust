#![allow(dead_code)]


use vector2::Vector2;
use crate::objects::ObjectId;

/* 
Stores information about a an object's linear transform. The object's hierarchy
children will be transformed relative to the parent object's transform.
For example, an anchor has a transform with position (50, 50) and it has a
child enemy with its own transform with position (0, 25). The anchor's transform
will get applied to the enemy and its 'real' (global) position will be (50, 75).
*/
pub struct Transform {
    pub position: Vector2,
    pub rotation: f32,
    pub scale: Vector2,
}

impl Transform {
    pub fn new(position: Vector2, rotation: f32, scale: Vector2) -> Transform {
        Transform { position, rotation, scale }
    }
    pub fn at_position(position: Vector2) -> Transform {
        Self::new(position, 1.0, Vector2::ONE)
    }
    
}

/*
Stores information about the place of an object in the scene tree hierarchy.
*/ 
pub struct Hierarchy {
    pub parent: Option<ObjectId>,
    pub children: Vec<ObjectId>,
}

impl Hierarchy {
    pub fn empty() -> Self {
        Self { parent: None, children: vec![] }
    }
    pub fn with_parent(parent: ObjectId) -> Self {
        Self { parent: Some(parent), children: vec![] }
    }
}