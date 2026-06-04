use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

use crate::canvas::Canvas;
use crate::vector::Vec3;

const GRID_SIZE: usize = 9;
const N_POINTS: usize = GRID_SIZE * GRID_SIZE * GRID_SIZE;

#[derive(Debug)]
struct App {
    context: Context<OwnedDisplayHandle>,
    state: AppState,
    cube_point: [Vec3; N_POINTS]
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

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let AppState::Running { window, .. } = &self.state {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let AppState::Running { window, surface } = &mut self.state else {
            return;
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
                if let (Some(width), Some(height)) =
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

                if let Ok(mut buffer) = surface.buffer_mut() {
                    let mut canvas = Canvas::new(&mut buffer, width, height);
                    canvas.clear(0xFF000000);
                    canvas.draw_grid(0xFF333333);
                    canvas.draw_rect(100, 100, 200, 150, 0xFFFF0000);

                    let _ = buffer.present();
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}

impl App {
    fn init_cube(&mut self) {
        let mut point_count: usize = 0;
        for xi in 0..GRID_SIZE {
            let x = -1.0 + xi as f32 * 0.25;
            for yi in 0..GRID_SIZE {
                let y = -1.0 + yi as f32 * 0.25;
                for zi in 0..GRID_SIZE {
                    let z = -1.0 + zi as f32 * 0.25;
                    self.cube_point[point_count] = Vec3 { x, y, z };
                    point_count += 1;
                }
            }
        }
    }
}

pub fn run_event_loop() {
    let event_loop = EventLoop::new().unwrap();
    let context = Context::new(event_loop.owned_display_handle()).unwrap();
    let mut app = App {
        context,
        state: AppState::Initial,
        cube_point: [Vec3::default(); N_POINTS]
    };
    app.init_cube();
    event_loop.run_app(&mut app).unwrap();
}
