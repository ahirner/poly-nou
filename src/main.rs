mod entity;
mod geometry;
mod render;

use nalgebra::RealField;
use nannou::prelude::*;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};

use crate::entity::Entity;
use crate::geometry::rand_poly;
use crate::render::Nannou;

struct Model<T: RealField = f32> {
    text: String,
    ent: Entity,
    world: PhysicsWorld<T>,
}

struct PhysicsWorld<T: RealField = f32> {
    bodies: DefaultBodySet<T>,
    colliders: DefaultColliderSet<T>,
    force_generators: DefaultForceGeneratorSet<T>,
    joint_constraints: DefaultJointConstraintSet<T>,
}

fn model(_app: &App) -> Model {
    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();

    let poly = rand_poly::<Point2>(30, 100.0, 15.0, 0.01);
    let ent = Entity::new(&mut colliders, &mut bodies, "aBc", poly, PURPLE, 1.0);

    let world = PhysicsWorld {
        bodies: bodies,
        colliders: colliders,
        force_generators: DefaultForceGeneratorSet::new(),
        joint_constraints: DefaultJointConstraintSet::new(),
    };

    Model { text: "Hello world!".to_owned(), ent: ent, world: world }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let win_rect = app.main_window().rect().pad(20.0);
    let text = model.text.as_str();
    draw.polygon();
    draw.text(text).color(WHITE).font_size(24).wh(win_rect.wh());

    model.ent.display(&draw, &model.world.bodies, &model.world.colliders);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
