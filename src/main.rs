#[macro_use] extern crate glium;
extern crate nannou;
extern crate rustfft;

use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

use nannou::ui::prelude::*;
use nannou::prelude::*;

fn main() {
    nannou::run(model, event, view);
}

struct Model {
    ui: Ui,
    ids: Ids,
    time_inc: f32,
    window: WindowId,
    time: f32,
    planner: FFTplanner<f32>,
    input: Vec<Complex<f32>>,
    output: Vec<Complex<f32>>,
}

struct Ids {
    time_inc: widget::Id,
}


fn model(app: &App) -> Model {
    let mut input:  Vec<Complex<f32>> = vec![Complex::zero(); 1234];
    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); 1234];

    let mut planner = FFTplanner::new(false);
    
    app.main_window().hide();
    let window = app.new_window().with_vsync(true).with_title("pivj").with_dimensions(800,600).build().unwrap();
    let mut ui = app.new_ui().window(window).build().unwrap();
    let ids = Ids {
        time_inc: ui.generate_widget_id(),
    };
    let time = 0.0;
    let time_inc = 0.2;
    Model {  ui,  ids, time_inc, window, time, planner, input, output}
}

fn event(_app: &App, mut model: Model, event: Event) -> Model {
    match event {
        Event::WindowEvent { simple: Some(event), .. } => match event {

            Moved(_pos) => {
            },

            KeyPressed(_key) => {
                if _key == Key::Space {
                    _app.main_window().set_fullscreen(Some(_app.main_window().current_monitor()));
                }
            },

            KeyReleased(_key) => {
            },

            MouseMoved(_pos) => {
            },

            MouseDragged(_pos, _button) => {
            },

            MousePressed(_button) => {
            },

            MouseReleased(_button) => {
            },

            MouseEntered => {
            },

            MouseExited => {
            },

            Resized(_size) => {
            },

            _other => (),
        },

        Event::Update(_dt) => {
            //let fft = model.planner.plan_fft(1234);
            //fft.process(&mut model.input, &mut model.output);
            
            //let mut sum = 0.0;
            //for x in &model.input {
             //   sum += x.norm();
            //}
            // println!("sum {}", sum);

            let ui = &mut model.ui.set_widgets();
            fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
                widget::Slider::new(val, min, max)
                    .w_h(200.0, 30.0)
                    .label_font_size(15)
                    .rgb(0.3, 0.3, 0.3)
                    .label_rgb(1.0, 1.0, 1.0)
                    .border(0.0)
            }

            for value in slider(model.time_inc as f32, 0.0, 1.0)
                .top_left_with_margin(20.0)
                .label("Resolution")
                .set(model.ids.time_inc, ui)
            {
                model.time_inc = value as f32;
            }

            model.time += model.time_inc;
        },

        _ => (),
    }
    model
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    // Our app only has one window, so retrieve this part of the `Frame`. Color it gray.
    frame.window(model.window).unwrap().clear_color(0.0, 0.0, 0.0, 1.0);

    let rect = nannou::geom::Rect::from_xy_wh(Point2 { x:0.0, y: 0.0 }, Vector2 {x: 2.0, y: 2.0 });
    // Get the 2 triangles to form a rectangle 
    let (tri_a, tri_b) = rect.triangles();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    let shape: Vec<Vertex> = tri_a.iter()
        .chain(tri_b.iter())
        .map(|p| Vertex { position: [p.x as f32, p.y as f32] })
        .collect();

    let win = app.window(model.window).unwrap();
    let display = win.inner_glium_display();
    let vertex_buffer = nannou::glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = nannou::glium::index::NoIndices(nannou::glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 330

        in vec2 position;
        out vec2 uv;

        void main() {
            uv = position;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330

        out vec4 color;
        in vec2 uv;

        uniform float iTime;

        void main() {
            color = vec4(abs(sin(abs(uv.x)+iTime*0.3)), cos(abs(uv.y)+iTime*0.05), sin(iTime*0.1), 1.0);
        }
    "#;


    let program = nannou::glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    frame.window(model.window).unwrap().draw(&vertex_buffer, &indices, &program, &uniform! { iTime: model.time },
            &Default::default()).unwrap();
    // Return the drawn frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
    frame
}