use crate::geometry::CommonPoint2;
use crate::render::Nannou;
use nannou::draw::properties::{ColorScalar, SetColor};
use nannou::prelude::*;
use ncollide2d::shape::{Polyline, ShapeHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle,
    DefaultColliderSet, RigidBodyDesc,
};

/// A shape with world position, body, annotation and other instance-specific info
pub struct Entity {
    pub label: Option<String>,
    pub base_color: Option<LinSrgba<f32>>,
    pub density: f32,
    pub body_handle: DefaultBodyHandle,
    pub collider_handle: DefaultColliderHandle,
}

impl Entity {
    pub fn new<I>(
        collider_set: &mut DefaultColliderSet<f32>,
        body_set: &mut DefaultBodySet<f32>,
        polygon: I,
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
            label: None,
            base_color: None,
            density: density,
            body_handle: body_handle,
            collider_handle: collider_handle,
        }
    }

    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_owned());
        self
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

        let draw_poly = draw
            .polyline()
            //.stroke(PINK)
            .stroke_weight(2.0)
            .join_round()
            .points(points);

        if let Some(c) = self.base_color {
            draw_poly.color(c);
        };

        let _draw_label = self.label.as_ref().map_or_else(
            || draw.text(&format!("{}-gon", shape.points().len())).color(GRAY),
            |s| draw.text(s).color(WHITE),
        );
    }
}

impl SetColor<ColorScalar> for Entity {
    fn rgba_mut(&mut self) -> &mut Option<LinSrgba<f32>> {
        SetColor::rgba_mut(&mut self.base_color)
    }
}
