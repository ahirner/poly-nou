use crate::geometry::CommonPoint2;
use crate::render::Nannou;
use nannou::color::IntoLinSrgba;
use nannou::draw::primitive::polygon::SetPolygon;
use nannou::draw::properties::{ColorScalar, SetColor};
use nannou::prelude::*;
use nphysics2d::nalgebra;
use nphysics2d::nalgebra::{Isometry2, RealField};
use nphysics2d::ncollide2d::shape::{ConvexPolygon, Cuboid, Polyline, ShapeHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle,
    DefaultColliderSet, Ground, RigidBody, RigidBodyDesc,
};
use std::borrow::Cow;

trait AutoLabel {
    fn gen_label(&self) -> Cow<str>;
}

impl<S: RealField> AutoLabel for ConvexPolygon<S> {
    fn gen_label(&self) -> Cow<str> {
        Cow::from(format!("{}-gon", self.points().len()))
    }
}

impl<S: RealField> AutoLabel for Cuboid<S> {
    fn gen_label(&self) -> Cow<str> {
        Cow::from("Cuboid")
    }
}

impl<S: RealField> AutoLabel for Polyline<S> {
    fn gen_label(&self) -> Cow<str> {
        Cow::from(format!("{}-line", self.points().len()))
    }
}

/// A shape with world position, body, annotation and other instance-specific info
pub struct Entity {
    label: Option<String>,
    base_color: Option<LinSrgba<f32>>,
    body_handle: DefaultBodyHandle,
    collider_handle: DefaultColliderHandle,
}

impl Entity {
    pub fn new_poly<I>(
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
        // Todo: make a faux convex shape for now, need compound shape of convex parts
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
        let half_width = rect.x.len() / 2.;
        let center_x = rect.x.middle();
        let half_height = rect.y.len() / 2.;
        let center_y = rect.y.middle();

        let ground_shape = Cuboid::new(nalgebra::Vector2::new(half_width, half_height));

        let shape_handle = ShapeHandle::new(ground_shape);
        let body_handle = bodies.insert(Ground::new());

        let co = ColliderDesc::new(shape_handle)
            .translation(CommonPoint2::new(center_x, center_y))
            .build(BodyPartHandle(body_handle, 0));

        let collider_handle = colliders.insert(co);

        Entity {
            label: None,
            base_color: None,
            body_handle: body_handle,
            collider_handle: collider_handle,
        }
    }

    /// Mutate body if entity has a one (i.e. no ground)
    pub fn map_body_mut<U, F: FnOnce(&mut RigidBody<f32>) -> U>(
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
    /// set user-defined base color
    /// akin to nannou::draw::drawing::SetColor
    pub fn set_color<C>(&mut self, color: C) -> &mut Self
    where
        C: IntoLinSrgba<ColorScalar>,
    {
        self.base_color = Some(color.into_lin_srgba());
        self
    }

    /// remove base color
    pub fn unset_color(&mut self) -> &mut Self {
        self.base_color = None;
        self
    }

    /// set user-defined label
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.into());
        self
    }

    /// remove label
    pub fn unset_label(&mut self) -> &mut Self {
        self.label = None;
        self
    }
}

impl Nannou for Entity {
    fn display(&self, draw: &Draw, colliders: &DefaultColliderSet<f32>) {
        let collider = colliders.get(self.collider_handle).unwrap();
        let dyn_shape = collider.shape();

        let pos: Isometry2<f32> = *collider.position();
        let center = pos.translation.vector.into_nannou();
        let rotation: f32 = pos.rotation.arg();

        // Todo: generalize
        if let Some(shape) = dyn_shape.as_shape::<ConvexPolygon<f32>>() {
            let draw_label = self.label.as_ref().map_or_else(
                || draw.text(shape.gen_label().as_ref()).color(GRAY),
                |s| draw.text(s).color(WHITE),
            );
            draw_label.xy(center);

            let points = shape.points().iter().map(CommonPoint2::into_nannou);
            let _draw_poly = draw
                .polygon() // primitive
                .no_fill() // styling ...
                .stroke_weight(2.0)
                .join_round()
                .map_ty(|p| if let Some(c) = self.base_color { p.stroke_color(c) } else { p })
                .rotate(rotation) // transform
                .xy(center)
                .points(points); // geometry
        } else if let Some(shape) = dyn_shape.as_shape::<Cuboid<f32>>() {
            let extents = (shape.half_extents() * 2.0).into_nannou();
            let _draw_cuboid = draw
                .rect() // primitive
                .map_ty(|p| if let Some(c) = self.base_color { p.color(c) } else { p }) // styling
                .rotate(rotation) // transform
                .xy(center)
                .wh(extents); // geometry

            let draw_label = self.label.as_ref().map_or_else(
                || draw.text(shape.gen_label().as_ref()).color(GRAY),
                |s| draw.text(s).color(WHITE),
            );
            draw_label.xy(center);
        } else {
            unimplemented!("Displaying shape not supported");
        };
    }
}
