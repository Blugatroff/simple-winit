use std::collections::HashMap;
pub use winit::event::MouseButton;
pub use winit::event::VirtualKeyCode;
use winit::event::{DeviceEvent, ElementState, MouseScrollDelta, WindowEvent};

#[derive(Clone)]
pub struct Input {
    keys: HashMap<VirtualKeyCode, bool>,
    last_keys: HashMap<VirtualKeyCode, bool>,
    mouse_buttons: HashMap<MouseButton, bool>,
    last_mouse_buttons: HashMap<MouseButton, bool>,
    keys_pressed: Vec<VirtualKeyCode>,
    keys_released: Vec<VirtualKeyCode>,
    mouse_buttons_pressed: Vec<MouseButton>,
    mouse_buttons_released: Vec<MouseButton>,
    mouse_diff: (f32, f32),
    mouse_position: (f32, f32),
    mouse_wheel: f32,
    resized: Option<(i32, i32)>,
    quit: bool,
    characters: String,
}
impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
impl Input {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            last_keys: HashMap::new(),
            mouse_buttons: HashMap::new(),
            last_mouse_buttons: HashMap::new(),
            keys_pressed: Vec::new(),
            keys_released: Vec::new(),
            mouse_buttons_pressed: Vec::new(),
            mouse_buttons_released: Vec::new(),
            mouse_diff: (0.0, 0.0),
            mouse_wheel: 0.0,
            resized: None,
            mouse_position: (0.0, 0.0),
            quit: false,
            characters: String::new(),
        }
    }
    pub fn step(&mut self) {
        self.mouse_diff = (0.0, 0.0);
        self.mouse_wheel = 0.0;
        self.resized = None;
        self.last_keys = self.keys.clone();
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.last_mouse_buttons = self.mouse_buttons.clone();
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();
    }
    pub fn process_device_event(&mut self, event: winit::event::DeviceEvent, is_focused: bool) {
        match event {
            DeviceEvent::Added => {}
            DeviceEvent::Removed => {}
            DeviceEvent::MouseMotion { delta } => {
                if is_focused {
                    self.mouse_diff = (
                        self.mouse_diff.0 + delta.0 as f32,
                        self.mouse_diff.1 + delta.1 as f32,
                    );
                }
            }
            DeviceEvent::MouseWheel { delta } => {
                if is_focused {
                    match delta {
                        MouseScrollDelta::LineDelta(_delta_x, delta_y) => {
                            self.mouse_wheel += delta_y;
                        }
                        MouseScrollDelta::PixelDelta(delta) => {
                            self.mouse_wheel += delta.y as f32;
                        }
                    }
                }
            }
            DeviceEvent::Motion { .. } => {}
            DeviceEvent::Button { .. } => {}
            DeviceEvent::Text { .. } => {}
            DeviceEvent::Key(_) => {}
        }
    }
    pub fn process_window_event(&mut self, event: winit::event::WindowEvent) -> bool {
        match event {
            WindowEvent::Resized(size) => {
                self.resized = Some((size.width as i32, size.height as i32));
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => return false,
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::ReceivedCharacter(character) => {
                self.characters.push(character);
            }
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(virtual_keycode) = input.virtual_keycode {
                    self.keys
                        .insert(virtual_keycode, input.state == ElementState::Pressed);
                    if input.state == ElementState::Pressed {
                        if let Some(last) = self.last_keys.get(&virtual_keycode) {
                            if !*last {
                                self.keys_pressed.push(virtual_keycode);
                            }
                        } else {
                            self.keys_pressed.push(virtual_keycode);
                        }
                    } else if let Some(last) = self.last_keys.get(&virtual_keycode) {
                        if *last {
                            self.keys_released.push(virtual_keycode);
                        }
                    }
                }
            }
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x as f32, position.y as f32);
            }
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { delta, .. } => {
                if let MouseScrollDelta::PixelDelta(delta) = delta {
                    self.mouse_wheel += delta.y as f32
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse_buttons
                    .insert(button, state == ElementState::Pressed);
                match state {
                    ElementState::Pressed => {
                        if let Some(was_pressed) = self.last_mouse_buttons.get_mut(&button) {
                            if !*was_pressed {
                                self.mouse_buttons_pressed.push(button);
                            }
                        } else {
                            self.mouse_buttons_pressed.push(button);
                        }
                    }
                    ElementState::Released => {
                        if let Some(was_pressed) = self.last_mouse_buttons.get_mut(&button) {
                            if *was_pressed {
                                self.mouse_buttons_released.push(button);
                            }
                        }
                    }
                }
            }
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
        }
        !self.quit
    }
    pub fn key_held(&self, key: VirtualKeyCode) -> bool {
        return if let Some(key) = self.keys.get(&key) {
            *key
        } else {
            false
        };
    }
    pub fn key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }
    pub fn key_released(&self, key: VirtualKeyCode) -> bool {
        self.keys_released.contains(&key)
    }
    pub fn button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }
    pub fn button_released(&self, button: MouseButton) -> bool {
        self.mouse_buttons_released.contains(&button)
    }
    pub fn button_held(&self, button: MouseButton) -> bool {
        return if let Some(pressed) = self.mouse_buttons.get(&button) {
            *pressed
        } else {
            false
        };
    }
    pub fn mouse_diff(&self) -> (f32, f32) {
        self.mouse_diff
    }
    pub fn wheel(&self) -> f32 {
        self.mouse_wheel
    }
    pub fn resized(&self) -> Option<(i32, i32)> {
        self.resized
    }
    pub fn quit(&mut self) {
        self.quit = true
    }
    pub fn get_characters(&mut self) -> String {
        let characters = self.characters.clone();
        self.characters.clear();
        characters
    }
}
