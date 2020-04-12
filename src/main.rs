mod entity;
use entity::{rand_poly, Entity};
use nalgebra::RealField;
use nannou::prelude::*;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use std::borrow::Borrow;

struct Model<T: RealField = f32> {
    text: String,
    bodies: DefaultBodySet<T>,
    colliders: DefaultColliderSet<T>,
    force_generators: DefaultForceGeneratorSet<T>,
    joint_constraints: DefaultJointConstraintSet<T>,

    ent: Entity,
}

/// Things that can be drawn to the screen
trait Nannou {
    fn display(
        &self,
        draw: &app::Draw,
        body_set: &DefaultBodySet<f32>,
        collider_set: &DefaultColliderSet<f32>,
    );
    fn update(&mut self) {}
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
        // hax
        let points: Vec<_> = shape
            .points()
            .into_iter()
            .map(|p| pt2(p.coords.as_slice()[0], p.coords.as_slice()[1]))
            .collect();

        draw.polyline()
            .color(self.base_color)
            //.stroke(PINK)
            .stroke_weight(2.0)
            .join_round()
            .points(points);
    }
}

fn model(_app: &App) -> Model {
    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();

    let poly = rand_poly::<Point2>(30, 100.0, 5.0, 0.1);
    let ent = Entity::new(&mut colliders, &mut bodies, "aBc", poly, PURPLE, 1.0);

    Model {
        text: "Hello world!".to_owned(),
        bodies: bodies,
        colliders: colliders,
        force_generators: DefaultForceGeneratorSet::new(),
        joint_constraints: DefaultJointConstraintSet::new(),
        ent: ent,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let win_rect = app.main_window().rect().pad(20.0);
    let text = model.text.borrow();
    draw.polygon();
    draw.text(text).color(WHITE).font_size(24).wh(win_rect.wh());

    model.ent.display(&draw, &model.bodies, &model.colliders);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
