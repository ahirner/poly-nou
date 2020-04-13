use crate::geometry::CommonPoint2;
use crate::render::Nannou;
use nalgebra::Isometry2;
use nannou::color::IntoLinSrgba;
use nannou::prelude::*;
use ncollide2d::shape::{ConvexPolygon, Cuboid, Polyline, ShapeHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle,
    DefaultColliderSet, Ground, RigidBody, RigidBodyDesc,
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
        let poly = ConvexPolygon::try_new(vec_vec)
            .expect("Could not form convex shape from polygon points");
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

    pub fn new_ground(
        colliders: &mut DefaultColliderSet<f32>,
        bodies: &mut DefaultBodySet<f32>,
        rect: &Rect,
    ) -> Self {
        let half_width = (rect.x.end - rect.x.start) / 2.;
        let center_x = (rect.x.start + rect.x.end) / 2.;
        let half_height = (rect.y.end - rect.y.start) / 2.;
        let center_y = (rect.y.start + rect.y.end) / 2.;

        let ground_shape =
            ShapeHandle::new(Cuboid::new(nalgebra::Vector2::new(half_width, half_height)));

        // Build a static ground body and add it to the body set.
        let ground_handle = bodies.insert(Ground::new());

        let co = ColliderDesc::new(ground_shape)
            .translation(pt2(center_x, center_y).into_nalgebra().coords)
            .build(BodyPartHandle(ground_handle, 0));
        // Add the collider to the collider set.
        let collider_handle = colliders.insert(co);

        Entity {
            label: None,
            base_color: Some(PURPLE.into_lin_srgba()),
            body_handle: ground_handle,
            collider_handle: collider_handle,
        }
    }

    /// Set body position if the entity has a body (i.e. no ground)
    pub fn map_body<U, F: FnOnce(&mut RigidBody<f32>) -> U>(
        &self,
        bodies: &mut DefaultBodySet<f32>,
        f: F,
    ) -> Option<U> {
        let maybe_body = bodies.rigid_body_mut(self.body_handle);

        match maybe_body {
            Some(body) => Some(f(body)),
            None => None,
        }
    }
}

impl Nannou for Entity {
    fn display(&self, draw: &Draw, colliders: &DefaultColliderSet<f32>) {
        let collider = colliders.get(self.collider_handle).unwrap();
        //let shape = collider.shape().as_shape::<Polyline<f32>>().unwrap();
        let dyn_shape = collider.shape();

        let pos: Isometry2<f32> = *collider.position();
        let center: nalgebra::Point2<f32> = pos.clone().translation.vector.into();

        // Todo: generalize
        if let Some(shape) = dyn_shape.as_shape::<ConvexPolygon<f32>>() {
            let draw_label = self.label.as_ref().map_or_else(
                || draw.text(&format!("{}-gon", shape.points().len())).color(GRAY),
                |s| draw.text(s).color(WHITE),
            );

            draw_label.xy(center.into_nannou());

            let points = shape.points().iter().map(|p| (pos * p).into_nannou());
            let draw_poly = draw.polyline().stroke_weight(2.0).join_round().points(points);
            if let Some(c) = self.base_color {
                draw_poly.color(c);
            };
        } else if let Some(shape) = dyn_shape.as_shape::<Cuboid<f32>>() {
            let half_extents = shape.half_extents();
            let draw_cuboid = draw
                .rect()
                .w(half_extents.x * 2.0)
                .h(half_extents.y * 2.0)
                .xy(center.into_nannou());
            if let Some(c) = self.base_color {
                draw_cuboid.color(c);
            };
        } else {
            unimplemented!("Displaying shape not supported");
        };
    }
}
