use std::collections::HashMap;
pub use winit::event::MouseButton;
use winit::event::{DeviceEvent, ElementState, MouseScrollDelta, WindowEvent};
pub use winit::event::{ScanCode, VirtualKeyCode};

#[derive(Copy, Clone)]
pub enum CursorGrabAction {
    None,
    Grab,
    Loose,
}

#[derive(Clone)]
pub struct Input {
    code_keys: HashMap<VirtualKeyCode, bool>,
    code_last_keys: HashMap<VirtualKeyCode, bool>,
    code_keys_pressed: Vec<VirtualKeyCode>,
    code_keys_released: Vec<VirtualKeyCode>,

    keys: HashMap<ScanCode, bool>,
    last_keys: HashMap<ScanCode, bool>,
    keys_pressed: Vec<ScanCode>,
    keys_released: Vec<ScanCode>,

    mouse_buttons: HashMap<MouseButton, bool>,
    last_mouse_buttons: HashMap<MouseButton, bool>,
    mouse_buttons_pressed: Vec<MouseButton>,
    mouse_buttons_released: Vec<MouseButton>,
    mouse_diff: (f32, f32),
    mouse_position: (f32, f32),
    mouse_wheel: f32,

    resized: Option<(i32, i32)>,
    quit: bool,
    cursor_action: CursorGrabAction,
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
            keys_pressed: Vec::new(),
            keys_released: Vec::new(),

            code_keys: HashMap::new(),
            code_last_keys: HashMap::new(),
            code_keys_pressed: Vec::new(),
            code_keys_released: Vec::new(),

            mouse_buttons: HashMap::new(),
            last_mouse_buttons: HashMap::new(),
            mouse_buttons_pressed: Vec::new(),
            mouse_buttons_released: Vec::new(),
            mouse_diff: (0.0, 0.0),
            mouse_wheel: 0.0,
            resized: None,
            mouse_position: (0.0, 0.0),
            characters: String::new(),
            quit: false,
            cursor_action: CursorGrabAction::None,
        }
    }
    pub fn step(&mut self) {
        self.mouse_diff = (0.0, 0.0);
        self.mouse_wheel = 0.0;
        self.resized = None;
        self.code_last_keys = self.code_keys.clone();
        self.code_keys_pressed.clear();
        self.code_keys_released.clear();
        self.last_mouse_buttons = self.mouse_buttons.clone();
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();
        self.cursor_action = CursorGrabAction::None;
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
    pub fn process_window_event(&mut self, event: &winit::event::WindowEvent) -> bool {
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
                self.characters.push(*character);
            }
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { input, .. } => {
                self.keys
                    .insert(input.scancode, input.state == ElementState::Pressed);
                if input.state == ElementState::Pressed {
                    if let Some(last) = self.last_keys.get(&input.scancode) {
                        if !*last {
                            self.keys_pressed.push(input.scancode);
                        }
                    } else {
                        self.keys_pressed.push(input.scancode);
                    }
                } else if let Some(last) = self.last_keys.get(&input.scancode) {
                    if *last {
                        self.keys_released.push(input.scancode);
                    }
                }

                if let Some(virtual_keycode) = input.virtual_keycode {
                    self.code_keys
                        .insert(virtual_keycode, input.state == ElementState::Pressed);
                    if input.state == ElementState::Pressed {
                        if let Some(last) = self.code_last_keys.get(&virtual_keycode) {
                            if !*last {
                                self.code_keys_pressed.push(virtual_keycode);
                            }
                        } else {
                            self.code_keys_pressed.push(virtual_keycode);
                        }
                    } else if let Some(last) = self.code_last_keys.get(&virtual_keycode) {
                        if *last {
                            self.code_keys_released.push(virtual_keycode);
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
                    .insert(*button, *state == ElementState::Pressed);
                match state {
                    ElementState::Pressed => {
                        if let Some(was_pressed) = self.last_mouse_buttons.get_mut(&button) {
                            if !*was_pressed {
                                self.mouse_buttons_pressed.push(*button);
                            }
                        } else {
                            self.mouse_buttons_pressed.push(*button);
                        }
                    }
                    ElementState::Released => {
                        if let Some(was_pressed) = self.last_mouse_buttons.get_mut(&button) {
                            if *was_pressed {
                                self.mouse_buttons_released.push(*button);
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
    pub fn key_held<T: Into<Key>>(&self, key: T) -> bool {
        let k: Key = key.into();
        match k {
            Key::Virtual(key) => {
                if let Some(key) = self.code_keys.get(&key) {
                    *key
                } else {
                    false
                }
            }
            Key::ScanCode(key) => {
                if let Some(key) = self.keys.get(&key) {
                    *key
                } else {
                    false
                }
            }
        }
    }
    pub fn key_pressed<T: Into<Key>>(&self, key: T) -> bool {
        let key: Key = key.into();
        match key {
            Key::Virtual(key) => self.code_keys_pressed.contains(&key),
            Key::ScanCode(key) => self.keys_pressed.contains(&key),
        }
    }
    pub fn key_released<T: Into<Key>>(&self, key: T) -> bool {
        let key: Key = key.into();
        match key {
            Key::Virtual(key) => self.code_keys_released.contains(&key),
            Key::ScanCode(key) => self.keys_released.contains(&key),
        }
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
    pub fn grab_cursor(&mut self, grab: bool) {
        self.cursor_action = if grab {
            CursorGrabAction::Grab
        } else {
            CursorGrabAction::Loose
        }
    }
    pub fn cursor_action(&self) -> CursorGrabAction {
        self.cursor_action
    }
    pub fn will_quit(&self) -> bool {
        self.quit
    }
}

impl From<ScanCode> for Key {
    fn from(k: ScanCode) -> Self {
        Self::ScanCode(k)
    }
}

impl From<VirtualKeyCode> for Key {
    fn from(k: VirtualKeyCode) -> Self {
        Self::Virtual(k)
    }
}

pub enum Key {
    Virtual(VirtualKeyCode),
    ScanCode(ScanCode),
}
