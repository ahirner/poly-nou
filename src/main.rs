use nannou::prelude::*;
use std::borrow::Borrow;

#[derive(Debug)]
struct Model {
    text: String,
}

fn model(_app: &App) -> Model {
    Model { text: "Hello world!".to_owned() }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let win_rect = app.main_window().rect().pad(20.0);
    let text = model.text.borrow();

    draw.text(text).color(WHITE).font_size(24).wh(win_rect.wh());
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
