mod entity;
mod geometry;
mod render;

use nalgebra::{Isometry2, RealField};
use nannou::color::{hsl, IntoLinSrgba};
use nannou::prelude::*;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};

use crate::entity::Entity;
use crate::geometry::rand_poly;
use crate::render::Nannou;
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use rand::{thread_rng, Rng};

struct Model<T: RealField = f32> {
    text: String,
    ents: Vec<Entity>,
    world: PhysicsWorld<T>,
}

struct PhysicsWorld<T: RealField = f32> {
    mechanical_world: DefaultMechanicalWorld<T>,
    geometrical_world: DefaultGeometricalWorld<T>,
    bodies: DefaultBodySet<T>,
    colliders: DefaultColliderSet<T>,
    force_generators: DefaultForceGeneratorSet<T>,
    joint_constraints: DefaultJointConstraintSet<T>,
}

impl PhysicsWorld {
    fn step(&mut self) {
        let mw = &mut self.mechanical_world;
        mw.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators,
        );
    }
}

fn model(app: &App) -> Model {
    // set to nphysics default tick time
    app.set_loop_mode(LoopMode::rate_fps(60.0));

    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();

    let mut rng = thread_rng();
    let window_rect = app.main_window().rect();

    let ents_iter = (0..20).map(|_i| {
        let poly = rand_poly::<Point2>(rng.gen_range(10, 30), 50.0, 20.0, 0.015);
        let hue = rng.gen_range(0.0, 1.0);

        let pos = Isometry2::translation(
            rng.gen_range(window_rect.x.start, window_rect.x.end),
            rng.gen_range(0.0f32, window_rect.y.end),
        );

        let mut ent = Entity::new(&mut colliders, &mut bodies, poly, 1.0);
        ent.base_color = Some(hsl(hue, 0.7, 0.5).into_lin_srgba()); // Todo: ergonomics
        ent.set_body_pos(pos, &mut bodies);

        ent
    });
    let mut ents: Vec<_> = ents_iter.collect();

    ents.push(Entity::new_ground(
        &mut colliders,
        &mut bodies,
        &window_rect.pad_top(window_rect.y.len() - 40.0),
    ));

    let world = PhysicsWorld {
        mechanical_world: DefaultMechanicalWorld::new(nalgebra::Vector2::new(0.0, -98.1)),
        geometrical_world: DefaultGeometricalWorld::new(),
        bodies: bodies,
        colliders: colliders,
        force_generators: DefaultForceGeneratorSet::new(),
        joint_constraints: DefaultJointConstraintSet::new(),
    };

    Model { text: "Hello poly-nou!".to_owned(), ents: ents, world: world }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.world.step()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let win_rect = app.main_window().rect().pad(20.0);
    let text = model.text.as_str();
    draw.text(text).align_text_top().color(WHITE).font_size(24).wh(win_rect.wh());

    for ent in model.ents.iter() {
        ent.display(&draw, &model.world.colliders);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
