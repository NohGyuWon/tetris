use super::*;

const INPUT_COUNT: usize = InputType::_Max as usize;

pub struct Input {
    previous: [bool; INPUT_COUNT],
    current: [bool; INPUT_COUNT]
}

impl Input {
    pub fn new() -> Self {
        Self {
            previous: [false; INPUT_COUNT],
            current: [false; INPUT_COUNT]
        }
    }

    pub fn set(&mut self, input_type: InputType, state: bool) {
        self.current[input_type as usize] = state
    }

    pub fn is_hold(&self, input_type: InputType) -> bool {
        self.current[input_type as usize]
    }

    pub fn pressed(&self, input_type: InputType) -> bool {
        let in_type = input_type as usize;
        !self.previous[in_type] && self.current[in_type]
    }

    pub fn released(&self, input_type: InputType) -> bool {
        let in_type = input_type as usize;
        self.previous[in_type] && !self.current[in_type]
    }

    pub fn update(&mut self) {
        for i in 0..INPUT_COUNT {
            self.previous[i] = self.current[i]
        }
    }
}