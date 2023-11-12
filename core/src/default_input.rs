use std::collections::VecDeque;

use crate::input::Input;
use crate::button;

pub struct DefaultInput {
	events: VecDeque<(button::Button, button::Event)>
}

impl DefaultInput {
	pub fn new() -> Self {
		DefaultInput {
			events: VecDeque::<(button::Button, button::Event)>::new()
		}
	}
}

impl Input for DefaultInput {
	fn get_input(&mut self) -> Option<(button::Button, button::Event)> {
		match self.events.len() > 0 {
			true => self.events.pop_front(),
			false => None
		}
	}

	fn press(&mut self, button: button::Button) {
		self.events.push_back((button, button::Event::Press));
	}

	fn release(&mut self, button: button::Button) {
		self.events.push_back((button, button::Event::Release));
	}
}
