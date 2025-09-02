#![allow(dead_code)]


use vector2::Vector2;
use crate::objects::ObjectId;

// Stores information about a linear transform, used primarily 
// for applying a parent object's transform to its children.
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

// Stores information about the place of an object in the scene tree hierarchy.
pub struct Hierarchy {
    pub parent: Option<ObjectId>,
    pub children: Vec<ObjectId>,
}

impl Hierarchy {
    pub fn empty() -> Hierarchy {
        Hierarchy { parent: None, children: Vec::new() }
    }
}