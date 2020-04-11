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

fn model(_app: &App) -> Model {
    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();

    let poly = rand_poly(20.0, 2.0, 30);
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
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
