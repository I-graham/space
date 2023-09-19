use super::*;
use cgmath::*;
use std::cell::Cell;

pub struct Ship {
	pos: Vector2<f32>,
	vel: Vector2<f32>,
	acc: Cell<Vector2<f32>>,
}

impl Ship {
	pub fn new() -> Self {
		Self {
			pos: vec2(100., 0.),
			vel: vec2(0., 35.),
			acc: vec2(0., 0.).into(),
		}
	}
}

impl GameObject for Ship {
	type Scene = World;
	type Action = Action;

	fn plan(&self, scene: &Self::Scene, _external: &External, _messenger: &Sender<Dispatch>) {
		self.acc
			.set(gravitational_acceleration(self, &scene.celestials))
	}

	fn update(&mut self, external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		self.pos += external.delta * self.vel;
		self.vel += external.delta * self.acc.get();

		None
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		Some(Instance {
			position: self.pos.into(),
			..external
				.instance(Texture::Ship)
				.nth_frame(0, Texture::Ship.frame_count())
		})
	}
}

impl Celestial for Ship {
	fn coords(&self) -> Vector2<f32> {
		self.pos
	}

	fn mass(&self) -> f32 {
		10.
	}

	fn radius(&self) -> f32 {
		32. * 2f32.sqrt()
	}
}
