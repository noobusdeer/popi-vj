extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {}

fn model(app: &App) -> Model {
    let _window = app.new_window().with_title("vj").build().unwrap();    
    app.main_window().set_fullscreen(Some(app.main_window().current_monitor()));
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

    let range = 1000.0;
    let bg = geom::Quad([
        pt2(-(range / 2.0), range / 2.0),
        pt2(range / 2.0, range / 2.0), 
        pt2(range / 2.0, -(range / 2.0)), 
        pt2(-(range / 2.0), -(range / 2.0))]);

    let tris = bg.triangles_iter().map(|tri| {
            tri.map_vertices(|v| {
                let color = Rgba::new(0.0, 1.0, 1.0, 1.0);
                geom::vertex::Rgba(v, color)
            })
        });
    draw.mesh().tris(tris);
    
    draw.to_frame(app, &frame).unwrap();
    frame
}