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
use nannou::draw::properties::SetColor;
use rand::{thread_rng, Rng};

struct Model<T: RealField = f32> {
    text: String,
    ents: Vec<Entity>,
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

    let mut rng = thread_rng();

    let ents_iter = (0..2).map(|_i| {
        let poly = rand_poly::<Point2>(rng.gen_range(10, 30), 100.0, 20.0, 0.015);
        let hue = rng.gen_range(0.0, 1.0);

        let ent = Entity::new(&mut colliders, &mut bodies, poly, 1.0).hsl(hue, 0.7, 0.5);
        ent
    });
    let ents: Vec<_> = ents_iter.collect();

    let world = PhysicsWorld {
        bodies: bodies,
        colliders: colliders,
        force_generators: DefaultForceGeneratorSet::new(),
        joint_constraints: DefaultJointConstraintSet::new(),
    };

    Model { text: "Hello poly-nou!".to_owned(), ents: ents, world: world }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let win_rect = app.main_window().rect().pad(20.0);
    let text = model.text.as_str();
    draw.polygon();
    draw.text(text).align_text_top().color(WHITE).font_size(24).wh(win_rect.wh());

    for ent in model.ents.iter() {
        ent.display(&draw, &model.world.bodies, &model.world.colliders);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
