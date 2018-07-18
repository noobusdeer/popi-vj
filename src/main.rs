extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {}

fn model(app: &App) -> Model {
    //let _monitor = nannou::MonitorId { /* fields */ };
    let _window = app.new_window()/*.with_fullscreen(_monitor)*/.with_title("vj").build().unwrap();
    Model {}
}

fn event(_app: &App, model: Model, event: Event) -> Model {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            Moved(_pos) => {}
            KeyPressed(_key) => {}
            KeyReleased(_key) => {}
            MouseMoved(_pos) => {}
            MouseDragged(_pos, _button) => {}
            MousePressed(_button) => {}
            MouseReleased(_button) => {}
            MouseEntered => {}
            MouseExited => {}
            Resized(_size) => {}
            _other => (),
        },

        Event::Update(_dt) => {}

        _ => (),
    }
    model
}

fn view(app: &App, _model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
    frame
}