use nannou::prelude::*;
use ncollide2d::shape::{Polyline, ShapeHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle,
    DefaultColliderSet, RigidBodyDesc,
};
use rand::prelude::*;
use rand_distr::StandardNormal;

pub trait CommonPoint2<S: nalgebra::RealField> {
    // Todo: use associated type?
    fn new(x: S, y: S) -> Self;
    fn get_x(&self) -> S;
    fn get_y(&self) -> S;

    fn into_nannou(&self) -> Point2<S> {
        pt2(self.get_x(), self.get_y())
    }

    fn into_nalgebra(&self) -> nalgebra::Point2<S> {
        nalgebra::Point2::new(self.get_x(), self.get_y())
    }
}

impl<S: nalgebra::RealField> CommonPoint2<S> for Point2<S> {
    fn new(x: S, y: S) -> Self {
        pt2(x, y)
    }
    fn get_x(&self) -> S {
        self.x
    }
    fn get_y(&self) -> S {
        self.y
    }
}

impl<S: nalgebra::RealField> CommonPoint2<S> for nalgebra::Point2<S> {
    fn new(x: S, y: S) -> Self {
        nalgebra::Point2::new(x, y)
    }
    fn get_x(&self) -> S {
        self.coords.as_slice()[0]
    }
    // Todo: use index instead?
    fn get_y(&self) -> S {
        self.coords.as_slice()[1]
    }
}

/// A shape with world position, body, annotation and other instance-specific info
pub struct Entity {
    label: String,
    base_color: Rgb<u8>,
    density: f32,
    body_handle: DefaultBodyHandle,
    collider_handle: DefaultColliderHandle,
}

pub fn rand_poly<T>(mean_rad: f32, std_rad: f32, n_verts: usize) -> impl Iterator<Item = T>
where
    T: CommonPoint2<f32>,
{
    let mut rng = thread_rng();

    assert!(n_verts >= 2);
    assert!(mean_rad > 0.0);

    let mut last_phase = 0.0f32;
    let points = (0..n_verts - 1).map(move |i| {
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

        T::new(x, y)
    });

    points
}

impl Entity {
    pub fn new<I>(
        collider_set: &mut DefaultColliderSet<f32>,
        body_set: &mut DefaultBodySet<f32>,
        label: &str,
        polygon: I,
        color: Rgb<u8>,
        density: f32,
    ) -> Self
    where
        I: IntoIterator,
        I::Item: CommonPoint2<f32>,
    {
        let body_builder = RigidBodyDesc::new();
        let body = body_builder.build();
        let body_handle = body_set.insert(body);

        let vec_vec: Vec<nalgebra::Point2<f32>> =
            polygon.into_iter().map(|p| p.into_nalgebra()).collect();

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
