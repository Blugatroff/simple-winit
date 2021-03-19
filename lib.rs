mod callback;
pub mod input;

use callback::process_device_event;
use callback::process_window_event;
pub use callback::InputEvent;
use input::Input;
use std::time::Duration;
pub use winit;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;
use winit::{event::Event, event_loop::EventLoop, window::Window};
use crate::input::CursorGrabAction;

pub trait WindowLoop {
    fn init(&mut self);
    fn update(&mut self, input: &mut Input, dt: Duration);
    fn render(&mut self);
    fn on_close(&mut self);
    fn input_event(&mut self, event: InputEvent);
}
pub fn create(name: &str) -> (Window, EventLoop<()>) {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_cursor_grab(true).ok();
    window.set_cursor_visible(false);
    window.set_title(name);
    (window, event_loop)
}
pub fn start<T: 'static + WindowLoop>(mut state: T, window: (Window, EventLoop<()>)) {
    let (window, event_loop) = window;
    let mut input_state = Input::new();
    let mut is_focused = true;
    let mut last_time = std::time::Instant::now();
    let mut is_grabbing_cursor = false;
    state.init();
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
            WindowEvent::Focused(true) => {
                if is_grabbing_cursor {
                    window.set_cursor_visible(!is_grabbing_cursor);
                    window.set_cursor_grab(true).ok();
                }
                is_focused = true;
            }
            WindowEvent::Focused(false) => {
                window.set_cursor_visible(true);
                window.set_cursor_grab(false).ok();
                is_focused = false;
            }
            _ => {
                if !input_state.process_window_event(&event) {
                    *control_flow = ControlFlow::Exit
                }
                if let Some(event) = process_window_event(event) {
                    state.input_event(event);
                }
            }
        },
        Event::DeviceEvent { event, .. } => {
            if is_focused {
                if let Some(event) = process_device_event(&event) {
                    state.input_event(event);
                }
            }
            input_state.process_device_event(event, is_focused);
        }
        Event::NewEvents(_) => {}
        Event::UserEvent(_) => {}
        Event::Suspended => {}
        Event::Resumed => {}
        Event::MainEventsCleared => {}
        Event::RedrawRequested(_) => {
            if input_state.will_quit() {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            let dt = last_time.elapsed();
            last_time = std::time::Instant::now();
            state.update(&mut input_state, dt);
            state.render();
            match input_state.cursor_action() {
                CursorGrabAction::None => {}
                CursorGrabAction::Grab => {
                    is_grabbing_cursor = true;
                    window.set_cursor_visible(false);
                    window.set_cursor_grab(true).ok();
                }
                CursorGrabAction::Loose => {
                    is_grabbing_cursor = false;
                    window.set_cursor_visible(true);
                    window.set_cursor_grab(false).ok();
                }
            }
            input_state.step();
        }
        Event::RedrawEventsCleared => {
            window.request_redraw();
        }
        Event::LoopDestroyed => {
            state.on_close();
        }
        Event::WindowEvent { .. } => {}
    });
}
