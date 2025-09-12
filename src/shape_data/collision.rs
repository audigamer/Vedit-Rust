#![allow(dead_code)]
use crate::shape_data::*;

pub fn rect_rect(a: &Rectangle, b: &Rectangle) -> bool {
    let intersects_horizontally: bool = a.bottom_right.x > b.top_left.x &&
                                        a.top_left.x < b.bottom_right.x;
    let intersects_vertically: bool = a.bottom_right.y > b.top_left.y &&
                                      a.top_left.y < b.bottom_right.y;

    intersects_horizontally && intersects_vertically
}

pub fn circle_circle(a: &Circle, b: &Circle) -> bool {
    let dx: f64 = a.center.x - b.center.x;
    let dy: f64 = a.center.y - b.center.y;
    let distance_squared: f64 = dx*dx + dy*dy;

    let radius_sum: f64 = a.radius + b.radius;

    distance_squared <= radius_sum * radius_sum
}

pub fn rect_circle(r: &Rectangle, c: &Circle) -> bool {
    // Clamps the circle's center to the nearest point on the rectangle
    let closest_x: f64 = c.center.x.clamp(r.top_left.x, r.bottom_right.x);
    let closest_y: f64 = c.center.y.clamp(r.top_left.y, r.bottom_right.y);

    let dx: f64 = c.center.x - closest_x;
    let dy: f64 = c.center.y - closest_y;

    let distance_squared = dx*dx + dy*dy;
    distance_squared <= c.radius * c.radius
}

// pub fn shape_shape(a: &Shape, b: &Shape) -> bool {
//     match (a, b) {
//             (Shape::Rectangle(r1), Shape::Rectangle(r2)) => rect_rect(r1, r2),
//             (Shape::Circle(c1), Shape::Circle(c2))       => circle_circle(c1, c2),
//             (Shape::Rectangle(r), Shape::Circle(c))      => rect_circle(r, c),
//             (Shape::Circle(c), Shape::Rectangle(r))      => rect_circle(r, c),
//         }
// }