use crate::geometry::CommonPoint2;
use crate::render::Nannou;
use nannou::prelude::*;
use ncollide2d::shape::{Polyline, ShapeHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle,
    DefaultColliderSet, RigidBodyDesc,
};

/// A shape with world position, body, annotation and other instance-specific info
pub struct Entity {
    pub label: String,
    pub base_color: Rgb<u8>,
    pub density: f32,
    pub body_handle: DefaultBodyHandle,
    pub collider_handle: DefaultColliderHandle,
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

impl Nannou for Entity {
    fn display(
        &self,
        draw: &app::Draw,
        body_set: &DefaultBodySet<f32>,
        collider_set: &DefaultColliderSet<f32>,
    ) {
        let _body = body_set.rigid_body(self.body_handle);
        let collider = collider_set.get(self.collider_handle).unwrap();
        let shape = collider.shape().as_shape::<ncollide2d::shape::Polyline<f32>>().unwrap();

        let points = shape.points().iter().map(CommonPoint2::into_nannou);

        draw.polyline()
            .color(self.base_color)
            //.stroke(PINK)
            .stroke_weight(2.0)
            .join_round()
            .points(points);
    }
}
