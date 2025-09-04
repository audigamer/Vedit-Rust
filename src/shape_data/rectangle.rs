use core::fmt;

use vector2::Vector2;

pub struct Rectangle {
    pub top_left: Vector2,
    pub bottom_right: Vector2,
}

impl Rectangle {
    pub fn new(top_left: Vector2, bottom_right: Vector2) -> Rectangle {
        if top_left.x >= bottom_right.x || top_left.y >= bottom_right.y {
            panic!("Invalid Vector2s, a top-left and a bottom-right Vector2 are needed.");
        }
        Rectangle {top_left, bottom_right}
    }

    pub fn from_center(position: Vector2, scale: Vector2) -> Rectangle {
        Rectangle::new(position - scale / 2.0, position + scale / 2.0)
    }

    #[allow(dead_code)]
    pub fn position(&self) -> Vector2 {
        Vector2 {
            x: (self.top_left.x + self.bottom_right.x) / 2.0,
            y: (self.top_left.y + self.bottom_right.y) / 2.0,
        }
    }

    pub fn scale(&self) -> Vector2 {
        Vector2 {
            x: self.bottom_right.x - self.top_left.x,
            y: self.bottom_right.y - self.top_left.y,
        }
    }

    // pub fn translate(&mut self, offset: Vector2) {
    //     self.top_left += offset;
    //     self.bottom_right += offset;
    // }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle(top_left: {}, bottom_right: {})", self.top_left, self.bottom_right)
    }
}