pub mod renderer;
pub mod utils;
pub mod window;
pub mod world;

use winit::{
    event::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode, MouseButton},
    event_loop::{EventLoop, ControlFlow},
    dpi::PhysicalSize
};
use pollster::block_on;
use crate::{
    window::EngineWindow,
    renderer::Renderer,
    world::World,
    utils::{math::Vec3f, ray::Ray}
};

pub struct Engine {
    pub window: EngineWindow,
    pub renderer: Renderer,
    pub world: World,
    pub camera: Camera,
    pub input: InputState,
}

pub struct Camera {
    pub position: Vec3f,
    pub rotation: Vec3f, // pitch, yaw, roll
    pub fov: f32,
    pub move_speed: f32,
    pub sensitivity: f32,
}

pub struct InputState {
    pub keys: Vec<VirtualKeyCode>,
    pub mouse_delta: (f64, f64),
    pub mouse_buttons: Vec<MouseButton>,
}

impl Engine {
    pub async fn new(title: &str, width: u32, height: u32) -> Self {
        let (window, event_loop) = EngineWindow::new(title, width, height);
        let renderer = Renderer::new(&window).await;
        let seed = 12345;
        let mut world = World::new(seed);
        
        Self {
            window,
            renderer,
            world,
            camera: Camera {
                position: Vec3f(0.0, 10.0, 20.0),
                rotation: Vec3f(0.0, 0.0, 0.0),
                fov: 75.0f32.to_radians(),
                move_speed: 5.0,
                sensitivity: 0.1,
            },
            input: InputState {
                keys: Vec::new(),
                mouse_delta: (0.0, 0.0),
                mouse_buttons: Vec::new(),
            },
        }
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new();
        
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => self.renderer.resize(size.width, size.height),
                    WindowEvent::KeyboardInput { input, .. } => self.handle_keyboard(input),
                    WindowEvent::MouseInput { button, state, .. } => self.handle_mouse(button, state),
                    WindowEvent::CursorMoved { position, .. } => self.handle_cursor(position),
                    _ => (),
                },
                Event::MainEventsCleared => {
                    self.update();
                    self.render();
                    self.window.window.request_redraw();
                },
                _ => (),
            }
        });
    }

    fn handle_keyboard(&mut self, input: KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            match input.state {
                ElementState::Pressed => self.input.keys.push(keycode),
                ElementState::Released => self.input.keys.retain(|&k| k != keycode),
            }
        }
    }

    fn handle_mouse(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => self.input.mouse_buttons.push(button),
            ElementState::Released => self.input.mouse_buttons.retain(|&b| b != button),
        }
    }

    fn handle_cursor(&mut self, position: winit::dpi::PhysicalPosition<f64>) {
        let center_x = self.window.size.0 as f64 / 2.0;
        let center_y = self.window.size.1 as f64 / 2.0;
        self.input.mouse_delta = (
            position.x - center_x,
            position.y - center_y
        );
    }

    fn update(&mut self) {
        let delta_time = 1.0 / 60.0;        
        
        self.camera.rotation.1 += self.input.mouse_delta.0 as f32 * self.camera.sensitivity * delta_time;
        self.camera.rotation.0 -= self.input.mouse_delta.1 as f32 * self.camera.sensitivity * delta_time;
        self.input.mouse_delta = (0.0, 0.0);

        let forward = Vec3f(
            self.camera.rotation.1.cos(),
            0.0,
            self.camera.rotation.1.sin()
        );

        let right = Vec3f(
            self.camera.rotation.1.sin(),
            0.0,
            -self.camera.rotation.1.cos()
        );

        let speed = self.camera.move_speed * delta_time;
        for key in &self.input.keys {
            match key {
                VirtualKeyCode::W => self.camera.position += forward * speed,
                VirtualKeyCode::S => self.camera.position -= forward * speed,
                VirtualKeyCode::A => self.camera.position -= right * speed,
                VirtualKeyCode::D => self.camera.position += right * speed,
                VirtualKeyCode::Space => self.camera.position.1 += speed,
                VirtualKeyCode::LShift => self.camera.position.1 -= speed,
                _ => (),
            }
        }

        self.world.update(delta_time);
    }

    /// current frame
    fn render(&mut self) {
        self.renderer.render(&self.world, &self.camera);
    }
}

pub fn run_engine() {

}

fn main() {
    println!("honeycomb, meet world. world, meet honeycomb.");
    block_on(async { Engine::new("honeycomb", 1280, 720).await.run(); });
}
