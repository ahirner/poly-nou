use nannou::app;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};

/// Things that can be drawn to the screen
pub trait Nannou {
    fn display(
        &self,
        draw: &app::Draw,
        body_set: &DefaultBodySet<f32>,
        collider_set: &DefaultColliderSet<f32>,
    );
    fn update(&mut self) {}
}
