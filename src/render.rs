use nannou::prelude::*;
use nphysics2d::object::DefaultColliderSet;

/// Things that can be drawn to the screen
pub trait Nannou {
    fn display(&self, draw: &Draw, collider_set: &DefaultColliderSet<f32>);
    fn update(&mut self) {}
}
