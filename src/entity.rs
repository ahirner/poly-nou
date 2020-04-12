use crate::geometry::CommonPoint2;
use crate::render::Nannou;
use nalgebra::Isometry2;
use nannou::prelude::*;
use ncollide2d::shape::{ConvexPolygon, Polyline, ShapeHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle,
    DefaultColliderSet, RigidBodyDesc,
};

/// A shape with world position, body, annotation and other instance-specific info
pub struct Entity {
    pub label: Option<String>,
    pub base_color: Option<LinSrgba<f32>>,
    body_handle: DefaultBodyHandle,
    collider_handle: DefaultColliderHandle,
}

impl Entity {
    pub fn new<I>(
        colliders: &mut DefaultColliderSet<f32>,
        bodies: &mut DefaultBodySet<f32>,
        polygon: I,
        density: f32,
    ) -> Self
    where
        I: IntoIterator,
        I::Item: CommonPoint2<f32>,
    {
        let body_builder = RigidBodyDesc::new();
        let body = body_builder.build();
        let body_handle = bodies.insert(body);

        let vec_vec: Vec<nalgebra::Point2<f32>> =
            polygon.into_iter().map(|p| p.into_nalgebra()).collect();

        // Todo: does Polyline require closed or open?
        //let poly = Polyline::new(vec_vec, None);
        // Todo: make a faux convex shape for now
        let poly = ConvexPolygon::try_new(vec_vec).unwrap();
        let shape = ShapeHandle::new(poly);

        let collider_desc = ColliderDesc::new(shape).density(density);
        let collider = collider_desc.build(BodyPartHandle(body_handle, 0));
        let collider_handle = colliders.insert(collider);

        Entity {
            label: None,
            base_color: None,
            body_handle: body_handle,
            collider_handle: collider_handle,
        }
    }

    pub fn set_body_pos(
        &mut self,
        pos: Isometry2<f32>,
        bodies: &mut DefaultBodySet<f32>,
    ) -> &mut Self {
        let body = bodies.rigid_body_mut(self.body_handle).unwrap();
        body.set_position(pos);
        self
    }
}

impl Nannou for Entity {
    fn display(
        &self,
        draw: &Draw,
        bodies: &DefaultBodySet<f32>,
        colliders: &DefaultColliderSet<f32>,
    ) {
        let _body = bodies.rigid_body(self.body_handle).unwrap();
        let collider = colliders.get(self.collider_handle).unwrap();
        //let shape = collider.shape().as_shape::<Polyline<f32>>().unwrap();
        let shape = collider.shape().as_shape::<ConvexPolygon<f32>>().unwrap();

        let pos: Isometry2<f32> = *collider.position();
        let center: nalgebra::Point2<f32> = pos.clone().translation.vector.into();
        let points = shape.points().iter().map(|p| (pos * p).into_nannou());

        let draw_poly = draw.polyline().stroke_weight(2.0).join_round().points(points);

        if let Some(c) = self.base_color {
            draw_poly.color(c);
        };

        let draw_label = self.label.as_ref().map_or_else(
            || draw.text(&format!("{}-gon", shape.points().len())).color(GRAY),
            |s| draw.text(s).color(WHITE),
        );

        draw_label.xy(center.into_nannou());
    }
}
