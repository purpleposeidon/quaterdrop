#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Program, Surface, VertexBuffer};
use glium::glutin::{ElementState, Event, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode, WindowBuilder};
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::{Uniforms, UniformValue};

#[derive(Clone, Copy, Debug)]
struct V { p: [f32; 2] }
impl std::fmt::Display for V {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({}, {})", self.p[0], self.p[1])
    }
}
implement_vertex!(V, p);

#[derive(Clone, Debug)]
struct DrawParams {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,

    width: f64,
    height: f64,

    mouse_x: f64,
    mouse_y: f64,

    maxi: f64,
    time: f64,
}
impl DrawParams {
    fn new(dims: (u32, u32)) -> DrawParams {
        DrawParams {
            x_min: -2.0,
            x_max: 1.0,
            y_min: -1.0,
            y_max: 1.0,
            width: dims.0 as f64,
            height: dims.1 as f64,
            mouse_x: 0.0,
            mouse_y: 0.0,
            maxi: 100.0,
            time: 0.0,
        }
    }
    fn reset(&mut self) {
        self.x_min = -2.0;
        self.x_max = 1.0;
        self.y_min = -1.0;
        self.y_max = 1.0;
    }
    fn scroll(&mut self, x: f64, y: f64) {
        let s_x = (self.x_max - self.x_min) / 10.0;
        let s_y = (self.y_max - self.y_min) / 10.0;
        self.x_min += x * s_x;
        self.x_max += x * s_x;
        self.y_min += y * s_y;
        self.y_max += y * s_y;
    }
    fn pan(&mut self, x: i32, y: i32) {
        self.scroll(x as f64 / 100.0,
                    y as f64 / 100.0)
    }
    fn zoom_in(&mut self) {
        let s_x = (self.x_max - self.x_min) / 10.0;
        let s_y = (self.y_max - self.y_min) / 10.0;
        self.x_min += s_x;
        self.x_max -= s_x;
        self.y_min += s_y;
        self.y_max -= s_y;
    }
    fn zoom_out(&mut self) {
        let s_x = (self.x_max - self.x_min) / 10.0;
        let s_y = (self.y_max - self.y_min) / 10.0;
        self.x_min -= s_x;
        self.x_max += s_x;
        self.y_min -= s_y;
        self.y_max += s_y;
    }
    fn clamp(&mut self) {
        self.maxi = self.maxi.max(1.0).min(1000.0);
    }
}
impl Uniforms for DrawParams {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("xMin", UniformValue::Double(self.x_min));
        f("xMax", UniformValue::Double(self.x_max));
        f("yMin", UniformValue::Double(self.y_min));
        f("yMax", UniformValue::Double(self.y_max));
        f("width", UniformValue::Double(self.width));
        f("height", UniformValue::Double(self.height));
        f("mouseX", UniformValue::Double(self.mouse_x));
        f("mouseY", UniformValue::Double(self.mouse_y));
        f("maxi", UniformValue::Double(self.maxi));
        f("time", UniformValue::Double(self.time));
    }
}

fn main() {
    // Initialize the window.
    let display = WindowBuilder::new()
        .with_multitouch()
        .with_title("quaterdrop")
        .with_vsync()
        .build_glium()
        .expect("couldn't open a window");
    // Store the vertices for a rectangle.
    let vertices = [
        V{ p: [1.0, -1.0] },
        V{ p: [-1.0, 1.0] },
        V{ p: [-1.0, -1.0] },
        V{ p: [1.0, 1.0] },
        V{ p: [1.0, -1.0] },
        V{ p: [-1.0, 1.0] },
    ];
    let vertex_buffer = VertexBuffer::new(&display, &vertices)
        .expect("couldn't init vertexbuffer");
    let indices = NoIndices(PrimitiveType::TrianglesList);
    // Load the GLSL program.
    let mut program = None;
    // Initialize the display parameters.
    let mut draw_params = DrawParams::new(display.get_window()
        .expect("couldn't get window")
        .get_inner_size_pixels()
        .expect("couldn't get window size"));

    // Input variables.
    let mut mouse_down = false;
    let mut zw_down = false;
    let mut mouse_last = (0, 0);
    let mut reload = false;

    // Main loop.
    loop {
        if program.is_none() || reload {
            fn read(name: &str) -> String {
                use std::fs::File;
                use std::io::Read;
                let mut fd = File::open(name).unwrap();
                let mut src = String::new();
                fd.read_to_string(&mut src).unwrap();
                src
            }
            let p = Program::from_source(
                &display,
                &read("src/vertex.glsl"),
                &read("src/fragment.glsl"),
                None
            );
            match p {
                Err(msg) => {
                    if !reload {
                        panic!("Couldn't compile program: {}", msg);
                    } else {
                        println!("Couldn't compile program: {}", msg);
                    }
                },
                Ok(p) => program = Some(p),
            }
            reload = false;
        }
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, program.as_ref().unwrap(), &draw_params, &Default::default())
            .expect("couldn't draw triangles");
        target.finish().expect("drawing failed");

        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => match code {
                    VirtualKeyCode::Minus => draw_params.zoom_out(),
                    VirtualKeyCode::Subtract => draw_params.zoom_out(),
                    VirtualKeyCode::Add => draw_params.zoom_in(),
                    VirtualKeyCode::Equals => draw_params.zoom_in(),
                    VirtualKeyCode::Space => draw_params.reset(),
                    VirtualKeyCode::Up => draw_params.scroll(0.0, -1.0),
                    VirtualKeyCode::Left => draw_params.scroll(-1.0, 0.0),
                    VirtualKeyCode::Right => draw_params.scroll(1.0, 0.0),
                    VirtualKeyCode::Down => draw_params.scroll(0.0, 1.0),
                    _ => (), //println!("Key: {:?}", code),
                },
                Event::MouseInput(state, MouseButton::Left) => mouse_down = match state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                },
                Event::MouseInput(state, MouseButton::Right) => zw_down = match state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                },
                Event::MouseMoved(x, y) => {
                    if mouse_down {
                        draw_params.pan(mouse_last.0 - x, mouse_last.1 - y);
                    } else if zw_down {
                        let dx = x - mouse_last.0;
                        let dy = y - mouse_last.1;
                        let w = draw_params.x_max - draw_params.x_min;
                        let h = draw_params.y_max - draw_params.y_min;
                        draw_params.mouse_x += dx as f64 * w * 0.001;
                        draw_params.mouse_y += dy as f64 * h * 0.001;
                    }
                    mouse_last = (x, y);
                },
                Event::MouseWheel(MouseScrollDelta::LineDelta(_x, y), TouchPhase::Moved) => {
                    if y < 0.0 {
                        draw_params.zoom_out()
                    } else {
                        draw_params.zoom_in()
                    }
                },
                Event::Resized(w, h) => {
                    draw_params.height = h as f64;
                    draw_params.width = w as f64;
                },
                Event::ReceivedCharacter('.') => {
                    println!("{:?}", draw_params);
                },
                Event::ReceivedCharacter('r') => {
                    println!("reload");
                    reload = true;
                },
                Event::ReceivedCharacter(c) => {
                    draw_params.maxi += match c {
                        'o' => -1.0,
                        'p' => 1.0,
                        '[' => -10.0,
                        ']' => 10.0,
                        _ => continue,
                    };
                    draw_params.clamp();
                    println!("max iterations: {}", draw_params.maxi);
                },
                _ => (),
                //_ => println!("Event: {:?}", ev),
            }
            //println!("{:?}, {:?}", (draw_params.x_min, draw_params.x_max), (draw_params.y_min, draw_params.y_max));
        }
        draw_params.time += 0.01;
    }
}
