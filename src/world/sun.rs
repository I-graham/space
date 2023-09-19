use cgmath::*;

use super::*;
use std::cell::Cell;

pub struct Sun {
	pos: Vector2<f32>,
	vel: Vector2<f32>,
	acc: Cell<Vector2<f32>>,
}

impl Sun {
	pub fn new() -> Self {
		Self {
			pos: vec2(0., 0.),
			vel: vec2(0., 0.),
			acc: vec2(0., 0.).into(),
		}
	}
}

impl GameObject for Sun {
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
			..external.instance(Texture::Sun)
		})
	}
}

impl Celestial for Sun {
	fn coords(&self) -> Vector2<f32> {
		self.pos
	}

	fn mass(&self) -> f32 {
		1000.
	}

	fn radius(&self) -> f32 {
		128. * 2f32.sqrt()
	}
}
