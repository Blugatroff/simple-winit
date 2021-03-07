use crate::input::MouseButton;
use winit::event::{ElementState, MouseScrollDelta, VirtualKeyCode, WindowEvent, DeviceEvent};

#[derive(Copy, Clone, Debug)]
pub enum InputEvent {
    MouseMotion((f64, f64)),
    MouseButton((u32, bool)),
    MouseWheel((f64, f64)),
    Character(char),
    Key((u32, bool, Option<VirtualKeyCode>)),
}

pub fn process_device_event(event: &winit::event::DeviceEvent) -> Option<InputEvent> {
    match event {
        DeviceEvent::Added => {}
        DeviceEvent::Removed => {}
        DeviceEvent::MouseMotion { delta } => {
            return Some(InputEvent::MouseMotion(*delta))
        }
        DeviceEvent::MouseWheel { .. } => {}
        DeviceEvent::Motion { .. } => {}
        DeviceEvent::Button { .. } => {}
        DeviceEvent::Key(_) => {}
        DeviceEvent::Text { .. } => {}
    }
    None
}

pub fn process_window_event(event: winit::event::WindowEvent) -> Option<InputEvent> {
    match event {
        WindowEvent::Resized(_) => None,
        WindowEvent::Moved(_) => None,
        WindowEvent::CloseRequested => None,
        WindowEvent::Destroyed => None,
        WindowEvent::DroppedFile(_) => None,
        WindowEvent::HoveredFile(_) => None,
        WindowEvent::HoveredFileCancelled => None,
        WindowEvent::ReceivedCharacter(char) => Some(InputEvent::Character(char)),
        WindowEvent::Focused(_) => None,
        WindowEvent::KeyboardInput { input, .. } => Some(InputEvent::Key((
            input.scancode,
            match input.state {
                ElementState::Pressed => true,
                ElementState::Released => false,
            },
            input.virtual_keycode,
        ))),
        WindowEvent::ModifiersChanged(_) => None,
        WindowEvent::CursorMoved { .. } => None,
        WindowEvent::CursorEntered { .. } => None,
        WindowEvent::CursorLeft { .. } => None,
        WindowEvent::MouseWheel { delta, .. } => match delta {
            MouseScrollDelta::LineDelta(_, _) => None,
            MouseScrollDelta::PixelDelta(delta) => Some(InputEvent::MouseWheel((delta.x, delta.y))),
        },
        WindowEvent::MouseInput { state, button, .. } => {
            let c = match button {
                MouseButton::Left => 0,
                MouseButton::Right => 1,
                MouseButton::Middle => 2,
                MouseButton::Other(c) => c,
            };
            Some(InputEvent::MouseButton((
                c as u32,
                match state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                },
            )))
        }
        WindowEvent::TouchpadPressure { .. } => None,
        WindowEvent::AxisMotion { .. } => None,
        WindowEvent::Touch(_) => None,
        WindowEvent::ScaleFactorChanged { .. } => None,
        WindowEvent::ThemeChanged(_) => None,
    }
}
