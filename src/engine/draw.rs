use macroquad::{color::*, shapes::*};
use vector2::Vector2;

use crate::shape_data::*;


// Shapes
pub fn draw_rect_shape(rect: &Rectangle, color: Color) {
    draw_rectangle(rect.top_left.x as f32, rect.top_left.y as f32,
        rect.scale().x as f32, rect.scale().y as f32, color);
}

pub fn draw_rect_outline(rect: &Rectangle, outline_width: f64, color: Color) {
    draw_rectangle_lines(rect.top_left.x as f32, rect.top_left.y as f32,
        rect.scale().x as f32, rect.scale().y as f32, outline_width as f32, color);
}

pub fn draw_outlined_rect(rect: &Rectangle, outline_width: f64, inner_color: Color, outer_color: Color) {
    draw_rect_outline(&rect, outline_width, outer_color);
    
    let inner_rect: Rectangle = Rectangle {
        top_left: rect.top_left + Vector2::ONE * outline_width / 2.0,
        bottom_right: rect.bottom_right - Vector2::ONE * outline_width / 2.0,
    };
    draw_rect_shape(&inner_rect, inner_color);
}

pub fn draw_circle_shape(circle: &Circle, color: Color) {
    draw_circle(circle.center.x as f32, circle.center.y as f32, circle.radius as f32, color);
}

pub fn draw_circle_outline(circle: &Circle, outline_width: f64, color: Color) {
    draw_circle_lines(circle.center.x as f32, circle.center.y as f32, circle.radius as f32, outline_width as f32, color);
}

pub fn draw_outlined_circle(circle: &Circle, outline_width: f64, inner_color: Color, outer_color: Color) {
    let inner_circle: Circle = Circle::new(circle.center, circle.radius - outline_width);
    let slighly_bigger_inner_circle: Circle = Circle::new(circle.center, inner_circle.radius + 0.5);
    draw_circle_outline(&inner_circle, outline_width, outer_color);
    draw_circle_shape(&slighly_bigger_inner_circle, inner_color);
}
