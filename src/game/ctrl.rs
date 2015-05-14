use math::{Vec2f, Vec2};
use sdl2::event::{Event, EventPump};
use sdl2::mouse::{self, Mouse};
use sdl2::scancode::ScanCode;
use std::vec::Vec;

pub type Sensitivity = f32;

pub enum Gesture {
    NoGesture,
    KeyHold(ScanCode),
    KeyTrigger(ScanCode),
    ButtonHold(Mouse),
    ButtonTrigger(Mouse),
    AnyOf(Vec<Gesture>),
    AllOf(Vec<Gesture>),
    QuitTrigger,
}

pub enum Analog2d {
    NoAnalog2d,

    // (mouse_sensitivity)
    Mouse(Sensitivity),

    // (xpos, xneg, ypos, yneg, step)
    Gestures(Gesture, Gesture, Gesture, Gesture, Sensitivity),
}

pub struct GameController<'sdl> {
    current_update_index: UpdateIndex,

    keyboard_state: [ButtonState; NUM_SCAN_CODES],
    quit_requested_index: UpdateIndex,

    mouse_enabled: bool,
    mouse_rel: Vec2f,

    pump: EventPump<'sdl>,
}

impl<'sdl> GameController<'sdl> {
    pub fn new(pump: EventPump<'sdl>) -> GameController<'sdl> {
        mouse::set_relative_mouse_mode(true);
        GameController {
            current_update_index: 1,
            keyboard_state: [ButtonState::Up(0); NUM_SCAN_CODES],
            quit_requested_index: 0,
            mouse_enabled: true,
            mouse_rel: Vec2::zero(),
            pump: pump,
        }
    }

    pub fn set_cursor_grabbed(&mut self, grabbed: bool) {
        mouse::set_relative_mouse_mode(grabbed);
    }

    pub fn set_mouse_enabled(&mut self, enable: bool) {
        self.mouse_enabled = enable;
    }

    pub fn update(&mut self) {
        self.current_update_index += 1;
        self.mouse_rel = Vec2::zero();
        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.quit_requested_index = self.current_update_index;
                },
                Event::KeyDown{ scancode, .. } => {
                    self.keyboard_state[scancode as usize] =
                        ButtonState::Down(self.current_update_index);
                },
                Event::KeyUp { scancode, .. } => {
                    self.keyboard_state[scancode as usize] =
                        ButtonState::Up(self.current_update_index);
                },
                Event::MouseMotion { xrel, yrel, .. } => {
                    if self.mouse_enabled {
                        self.mouse_rel = Vec2::new(xrel as f32, -yrel as f32);
                    } else {
                        self.mouse_rel = Vec2::zero();
                    }
                },
                _ => {}
            }
        }
    }

    pub fn poll_gesture(&self, gesture: &Gesture) -> bool {
        match *gesture {
            Gesture::QuitTrigger => {
                self.quit_requested_index == self.current_update_index
            },
            Gesture::KeyHold(code) => match self.keyboard_state[code as usize] {
                ButtonState::Down(_) => true,
                _ => false
            },
            Gesture::KeyTrigger(code)
                    => match self.keyboard_state[code as usize] {
                ButtonState::Down(index) => self.current_update_index == index,
                _ => false
            },
            Gesture::AnyOf(ref subs) => {
                for subgesture in subs.iter() {
                    if self.poll_gesture(subgesture) {
                        return true;
                    }
                }
                false
            },
            Gesture::AllOf(ref subs) => {
                for subgesture in subs.iter() {
                    if !self.poll_gesture(subgesture) {
                        return false;
                    }
                }
                true
            },
            Gesture::NoGesture => false,
            _ => { panic!("Unimplemented gesture type."); }
        }
    }

    pub fn poll_analog2d(&self, motion: &Analog2d) -> Vec2f {
        match motion {
            &Analog2d::Mouse(sensitivity) => self.mouse_rel * sensitivity,
            &Analog2d::Gestures(
                    ref xpos, ref xneg, ref ypos, ref yneg, step) => {
                Vec2::new(
                    if self.poll_gesture(xpos) { step }
                    else if self.poll_gesture(xneg) { -step }
                    else { 0.0 },
                    if self.poll_gesture(ypos) { step }
                    else if self.poll_gesture(yneg) { -step }
                    else { 0.0 }
                )
            }
            &Analog2d::NoAnalog2d => Vec2::zero()
        }
    }
}

const NUM_SCAN_CODES: usize = 512;

type UpdateIndex = u32;

#[derive(Copy, Clone)]
enum ButtonState {
    Up(UpdateIndex),
    Down(UpdateIndex),
}
