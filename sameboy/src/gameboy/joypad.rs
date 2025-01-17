use std::time::Duration;
use sameboy_sys::{GB_key_t, GB_key_t_GB_KEY_RIGHT, GB_key_t_GB_KEY_LEFT, GB_key_t_GB_KEY_UP, GB_key_t_GB_KEY_DOWN, GB_key_t_GB_KEY_A, GB_key_t_GB_KEY_B, GB_key_t_GB_KEY_SELECT, GB_key_t_GB_KEY_START, GB_set_key_state, GB_key_mask_t, GB_key_mask_t_GB_KEY_RIGHT_MASK, GB_key_mask_t_GB_KEY_LEFT_MASK, GB_key_mask_t_GB_KEY_UP_MASK, GB_key_mask_t_GB_KEY_DOWN_MASK, GB_key_mask_t_GB_KEY_A_MASK, GB_key_mask_t_GB_KEY_B_MASK, GB_key_mask_t_GB_KEY_SELECT_MASK, GB_key_mask_t_GB_KEY_START_MASK, GB_set_key_mask, GB_set_emulate_joypad_bouncing};
use crate::Model;
use super::Gameboy;

#[derive(Copy, Clone)]
pub enum JoypadButton {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start
}

impl From<GB_key_t> for JoypadButton {
    fn from(value: GB_key_t) -> Self {
        match value {
            GB_key_t_GB_KEY_RIGHT => Self::Right,
            GB_key_t_GB_KEY_LEFT => Self::Left,
            GB_key_t_GB_KEY_UP => Self::Up,
            GB_key_t_GB_KEY_DOWN => Self::Down,
            GB_key_t_GB_KEY_A => Self::A,
            GB_key_t_GB_KEY_B => Self::B,
            GB_key_t_GB_KEY_SELECT => Self::Select,
            GB_key_t_GB_KEY_START => Self::Start,
            _ => unreachable!("Invalid GB_key_t value")
        }
    }
}

impl From<JoypadButton> for GB_key_t {
    fn from(value: JoypadButton) -> Self {
        match value {
            JoypadButton::Right => GB_key_t_GB_KEY_RIGHT,
            JoypadButton::Left => GB_key_t_GB_KEY_LEFT,
            JoypadButton::Up => GB_key_t_GB_KEY_UP,
            JoypadButton::Down => GB_key_t_GB_KEY_DOWN,
            JoypadButton::A => GB_key_t_GB_KEY_A,
            JoypadButton::B => GB_key_t_GB_KEY_B,
            JoypadButton::Select => GB_key_t_GB_KEY_SELECT,
            JoypadButton::Start => GB_key_t_GB_KEY_START
        }
    }
}

fn mask_from_button(value: JoypadButton) -> GB_key_mask_t {
    match value {
        JoypadButton::Right => GB_key_mask_t_GB_KEY_RIGHT_MASK,
        JoypadButton::Left => GB_key_mask_t_GB_KEY_LEFT_MASK,
        JoypadButton::Up => GB_key_mask_t_GB_KEY_UP_MASK,
        JoypadButton::Down => GB_key_mask_t_GB_KEY_DOWN_MASK,
        JoypadButton::A => GB_key_mask_t_GB_KEY_A_MASK,
        JoypadButton::B => GB_key_mask_t_GB_KEY_B_MASK,
        JoypadButton::Select => GB_key_mask_t_GB_KEY_SELECT_MASK,
        JoypadButton::Start => GB_key_mask_t_GB_KEY_START_MASK
    }
}

impl Gameboy {
    /// Press or release a button on the joypad.
    pub fn set_joypad_button(&mut self, button: JoypadButton, pressed: bool) {
        unsafe {
            GB_set_key_state(self.as_mut_ptr(), button.into(), pressed);
        }
    }

    /// Release all buttons on the joypad.
    pub fn joypad_release_all(&mut self) {
        unsafe {
            GB_set_key_mask(self.as_mut_ptr(), 0 as GB_key_mask_t);
        }
    }

    /// Press only the buttons in the list, and run the GameBoy for a frame.
    /// Useful for automation.
    pub fn joypad_macro_press(&mut self, buttons: &[JoypadButton], duration: Option<Duration>) {
        let mask = buttons.iter()
            .fold(0 as GB_key_mask_t, |mask, &button| { mask | mask_from_button(button) });

        unsafe {
            GB_set_key_mask(self.as_mut_ptr(), mask);
        }

        let mut frames = (duration.unwrap_or(Duration::from_millis(100)).as_secs_f64() * 60.0) as usize;
        if duration.is_some() {
            match self.model() {
                Model::DMG(_) | Model::MGB | Model::SGB(_, _) | Model::SGB2(_) => frames *= 2,
                _ => ()
            }
        }

        for _ in 0..frames {
            self.run_frame();
        }
    }

    pub fn emulate_joypad_bouncing(&mut self, bouncing: bool) {
        unsafe {
            GB_set_emulate_joypad_bouncing(self.as_mut_ptr(), bouncing);
        }
    }
}
