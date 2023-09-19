use super::*;

use crate::window::Window;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;

pub struct GameState<World: Root> {
	pub(super) win: Window,
	messenger: Messenger,
	world: World,
}

impl<World: Root> GameState<World> {
	pub fn new(event_loop: &EventLoop<()>) -> Self {
		let api = Window::new::<World::Texture>(event_loop);
		Self {
			world: World::init(api.inputs()),
			messenger: Messenger::new(),
			win: api,
		}
	}

	pub fn step(&mut self) {
		self.world
			.plan(&(), self.win.inputs(), &self.messenger.sender());
		self.world.update(self.win.inputs(), &self.messenger);

		let now = std::time::Instant::now();
		self.win.inputs_mut().update(now);
		self.messenger.update(now);

		const CAM_MOVE_SPEED: f32 = 50.;

		self.win.inputs_mut().camera.pos.x += CAM_MOVE_SPEED
			* self.win.inputs().delta
			* (self.win.inputs().key(VirtualKeyCode::D).is_down() as i32
				- self.win.inputs().key(VirtualKeyCode::A).is_down() as i32) as f32;

		self.win.inputs_mut().camera.pos.y += CAM_MOVE_SPEED
			* self.win.inputs().delta
			* (self.win.inputs().key(VirtualKeyCode::W).is_down() as i32
				- self.win.inputs().key(VirtualKeyCode::S).is_down() as i32) as f32;

		const CAM_SCALE_SPEED: f32 = 50.;

		self.win.inputs_mut().camera.scale += CAM_SCALE_SPEED
			* self.win.inputs().delta
			* (self.win.inputs().key(VirtualKeyCode::Q).is_down() as i32
				- self.win.inputs().key(VirtualKeyCode::Z).is_down() as i32) as f32;
	}

	pub fn draw(&mut self) {
		self.win.clear();
		self.world.render(&mut self.win);
		self.win.draw();
	}

	pub fn cleanup(&mut self) {
		self.world.cleanup()
	}
}
