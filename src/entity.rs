use nannou::prelude::*;
use ncollide2d::shape::{Polyline, ShapeHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle,
    DefaultColliderSet, RigidBodyDesc,
};
use rand::prelude::*;
use rand_distr::StandardNormal;

/// A shape with world position, body, annotation and other instance-specific info
pub struct Entity {
    label: String,
    base_color: Rgb<u8>,
    density: f32,
    body_handle: DefaultBodyHandle,
    collider_handle: DefaultColliderHandle,
}

/// Malleable specification of a contour, does not include a closing Point2
pub type Polygon2 = Vec<Point2<f32>>;

pub fn rand_poly(mean_rad: f32, std_rad: f32, n_verts: usize) -> Polygon2 {
    let mut rng = thread_rng();

    assert!(n_verts >= 2);
    assert!(mean_rad > 0.0);

    let mut last_phase = 0.0f32;
    let points = (0..n_verts - 1).map(|i| {
        // angle
        let fract: f32 = i as f32 / n_verts as f32; // one less for closing
        let rand_phase: f32 = rng.sample(StandardNormal);
        let phase = (fract + rand_phase * 0.5).max(last_phase);
        last_phase = phase;

        // radius
        let rand_rad: f32 = rng.sample(StandardNormal);
        let rad: f32 = (mean_rad + rand_rad * std_rad).max(0.0f32);

        let x = rad * (TAU * phase).cos();
        let y = rad * (TAU * phase).sin();

        pt2(x, y)
    });

    points.collect()
}

impl Entity {
    pub fn new(
        collider_set: &mut DefaultColliderSet<f32>,
        body_set: &mut DefaultBodySet<f32>,
        label: &str,
        polygon: Polygon2,
        color: Rgb<u8>,
        density: f32,
    ) -> Self {
        let body_builder = RigidBodyDesc::new();
        let body = body_builder.build();
        let body_handle = body_set.insert(body);

        // Todo: should be more efficient, not back and forth conversion
        let vec_vec: Vec<nalgebra::Point2<f32>> =
            polygon.into_iter().map(|p| nalgebra::Point2::new(p.x, p.y)).collect();

        // Todo: does Polyline require closed or open?
        let poly_line = Polyline::new(vec_vec, None);
        let shape = ShapeHandle::new(poly_line);

        let collider_desc = ColliderDesc::new(shape);
        let collider = collider_desc.build(BodyPartHandle(body_handle, 0));
        let collider_handle = collider_set.insert(collider);

        Entity {
            label: label.to_owned(),
            base_color: color,
            density: density,
            body_handle: body_handle,
            collider_handle: collider_handle,
        }
    }
}
