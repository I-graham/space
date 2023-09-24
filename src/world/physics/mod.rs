mod gravity;
mod collisions;

pub use gravity::*;
pub use collisions::*;

use super::*;
use cgmath::*;

use std::cell::Cell;

#[derive(Clone, Debug)]
pub struct Physics {
	pub mass: f32,
	pub diameter: f32,
	pub pos: Vector2<f32>,
	pub vel: Vector2<f32>,
	pub ang: f32,
	pub ang_vel: f32,
	acc: Cell<Vector2<f32>>,
	ang_acc: Cell<f32>,
}

impl Physics {
	pub fn rest(mass: f32, diameter: f32) -> Self {
		Self {
			mass,
			diameter,
			pos: vec2(0., 0.),
			vel: vec2(0., 0.),
			acc: vec2(0., 0.).into(),
			ang: 0.,
			ang_vel: 0.,
			ang_acc: 0f32.into(),
		}
	}

	pub fn at(self, pos: Vector2<f32>) -> Self {
		Self { pos, ..self }
	}

	pub fn heading(self, vel: Vector2<f32>) -> Self {
		Self { vel, ..self }
	}

	pub fn spinning(self, ang_vel: f32) -> Self {
		Self { ang_vel, ..self }
	}

	pub fn apply_force(&self, force: Vector2<f32>) {
		self.acc.set(self.acc.get() + force / self.mass);
	}

	pub fn update(&mut self, delta: f32) {
		self.pos += delta * self.vel;
		self.vel += delta * self.acc.get();
		self.acc.set(vec2(0., 0.));

		self.ang += delta * self.ang_vel;
		self.ang_vel += delta * self.ang_acc.get();
		self.ang_acc.set(0.);
	}

	pub fn instance(&self, texture: Texture, external: &External) -> Instance {
		const TO_DEG: f32 = 180. / std::f32::consts::PI;
		Instance {
			position: self.pos.into(),
			scale: vec2(self.diameter, self.diameter).into(),
			rotation: (self.ang * TO_DEG).into(),
			..external.instance(texture)
		}
	}
}
