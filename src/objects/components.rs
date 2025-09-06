#![allow(dead_code)]


use vector2::Vector2;
use crate::{objects::ObjectId, shape_data::{circle::Circle, rectangle::Rectangle}};

/* 
Stores information about a an object's linear transform. The object's hierarchy
children will be transformed relative to the parent object's transform.
For example, an anchor has a transform with position (50, 50) and it has a
child enemy with its own transform with position (0, 25). The anchor's transform
will get applied to the enemy and its 'real' (global) position will be (50, 75).
*/
#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vector2,
    pub rotation: f32,
    pub scale: Vector2,
} // TODO: Make rotation and scale do something. Right now, only positions get applied.

impl Transform {
    pub fn new(position: Vector2, rotation: f32, scale: Vector2) -> Transform {
        Transform { position, rotation, scale }
    }
    pub fn at_position(position: Vector2) -> Transform {
        Self::new(position, 1.0, Vector2::ONE)
    }
    pub fn move_by(&mut self, offset: Vector2) {
        self.position += offset;
    }
    pub fn apply_transform(self, other: Transform) -> Transform {
        Transform {
            position: self.position + other.position,
            rotation: self.rotation + other.rotation,
            scale: Vector2::scale(self.scale, other.scale)
        }
    }

    /*pub fn transform_vector(&self, vector: Vector2)*/

    pub fn transform_rect(&self, rect: &Rectangle) -> Rectangle {
        // TODO: Implement rotation and scale.
        Rectangle {
            top_left: rect.top_left + self.position,
            bottom_right: rect.bottom_right + self.position,
        }
    }

    pub fn transform_circle(&self, circle: &Circle) -> Circle {
        Circle {
            center: circle.center + self.position,
            radius: circle.radius,
        }
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