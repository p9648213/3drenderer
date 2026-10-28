use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::path::Path;
use std::rc::Rc;
use std::time;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

use crate::canvas::Canvas;
use crate::mesh::Mesh;
use crate::triangle::Triangle;
use crate::vector::{Vec2, Vec3, vec3_rotate_x, vec3_rotate_y, vec3_rotate_z};

const FPS: u64 = 30;
const FRAME_TARGET_TIME: time::Duration = time::Duration::from_millis(1000 / FPS);

#[derive(Debug)]
struct App {
    context: Context<OwnedDisplayHandle>,
    state: AppState,
    fov_factor: f32,
    camera_position: Vec3,
    previous_frame_time: time::Instant,
    mesh: Mesh,
    triangle_to_render: Vec<Triangle>,
}

#[derive(Debug)]
enum AppState {
    Initial,
    Running {
        window: Rc<Window>,
        surface: Surface<OwnedDisplayHandle, Rc<Window>>,
    },
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let AppState::Initial = self.state {
            let window_attrs = Window::default_attributes().with_maximized(true);
            let window = event_loop
                .create_window(window_attrs)
                .expect("failed creating window");
            let window = Rc::new(window);
            let mut surface =
                Surface::new(&self.context, window.clone()).expect("failed creating surface");

            let size = window.inner_size();
            if let (Some(width), Some(height)) =
                (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
            {
                surface.resize(width, height).unwrap();
            }

            self.state = AppState::Running { window, surface };
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let AppState::Running { window, .. } = &self.state {
            window.request_redraw();
            event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
                self.previous_frame_time + FRAME_TARGET_TIME,
            ));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = match &self.state {
            AppState::Running { window, .. } => window.clone(),
            AppState::Initial => return,
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let AppState::Running { surface, .. } = &mut self.state
                    && let (Some(width), Some(height)) =
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    surface.resize(width, height).unwrap();
                }
            }
            WindowEvent::RedrawRequested => {
                let size = window.inner_size();
                let width = size.width as usize;
                let height = size.height as usize;

                if width == 0 || height == 0 {
                    return;
                }

                self.update(width, height);
                self.render(width, height);
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}

impl App {
    fn render(&mut self, width: usize, height: usize) {
        if let AppState::Running { surface, .. } = &mut self.state
            && let Ok(mut buffer) = surface.buffer_mut()
        {
            let mut canvas = Canvas::new(&mut buffer, width, height);
            canvas.clear(0xFF000000);
            canvas.draw_grid(0xFF333333);

            (0..self.triangle_to_render.len()).for_each(|i| {
                let triangle = &self.triangle_to_render[i];

                canvas.draw_rect(
                    triangle.point[0].x as usize,
                    triangle.point[0].y as usize,
                    3,
                    3,
                    0xFFFFFF00,
                );
                canvas.draw_rect(
                    triangle.point[1].x as usize,
                    triangle.point[1].y as usize,
                    3,
                    3,
                    0xFFFFFF00,
                );
                canvas.draw_rect(
                    triangle.point[2].x as usize,
                    triangle.point[2].y as usize,
                    3,
                    3,
                    0xFFFFFF00,
                );

                canvas.draw_triangle(
                    triangle.point[0].x as usize,
                    triangle.point[0].y as usize,
                    triangle.point[1].x as usize,
                    triangle.point[1].y as usize,
                    triangle.point[2].x as usize,
                    triangle.point[2].y as usize,
                    0xFF00FF00,
                );
            });

            let _ = buffer.present();
        }
    }

    fn update(&mut self, width: usize, height: usize) {
        let now = time::Instant::now();

        if now - self.previous_frame_time < FRAME_TARGET_TIME {
            return;
        }

        self.previous_frame_time = now;

        self.triangle_to_render = Vec::new();

        self.mesh.rotation.x += 0.01;
        self.mesh.rotation.y += 0.01;
        self.mesh.rotation.z += 0.01;

        for i in 0..self.mesh.faces.len() {
            let mesh_face = &self.mesh.faces[i];

            let mut face_vertices: [Vec3; 3] = [Vec3::default(), Vec3::default(), Vec3::default()];

            face_vertices[0] = self.mesh.vertices[(mesh_face.a - 1) as usize];
            face_vertices[1] = self.mesh.vertices[(mesh_face.b - 1) as usize];
            face_vertices[2] = self.mesh.vertices[(mesh_face.c - 1) as usize];

            let mut projected_triangle = Triangle::default();

            (0..3).for_each(|j| {
                let mut transformed_vertex = face_vertices[j];
                transformed_vertex = vec3_rotate_x(transformed_vertex, self.mesh.rotation.x);
                transformed_vertex = vec3_rotate_y(transformed_vertex, self.mesh.rotation.y);
                transformed_vertex = vec3_rotate_z(transformed_vertex, self.mesh.rotation.z);

                transformed_vertex.z -= self.camera_position.z;

                let mut projected_points = self.perspective_project(transformed_vertex);

                projected_points.x += width as f32 / 2.0;
                projected_points.y += height as f32 / 2.0;

                projected_triangle.point[j] = projected_points;
            });

            self.triangle_to_render.push(projected_triangle);
        }
    }

    fn perspective_project(&self, point: Vec3) -> Vec2 {
        Vec2 {
            x: self.fov_factor * (point.x / point.z),
            y: self.fov_factor * (point.y / point.z),
        }
    }
}

#[rustfmt::skip]
pub fn run_event_loop() {
    let event_loop = EventLoop::new().unwrap();
    let context = Context::new(event_loop.owned_display_handle()).unwrap();

    let mut app = App {
        context,
        state: AppState::Initial,
        fov_factor: 640.0,
        camera_position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -5.0,
        },
        previous_frame_time: std::time::Instant::now(),
        mesh: Mesh::default(),
        triangle_to_render: Vec::new(),
    };

    app.mesh.load_obj_file_data(Path::new("assets/f22.obj"));

    event_loop.run_app(&mut app).unwrap();
}
