use super::*;
use cgmath::*;

use std::cell::Cell;

pub struct Physics {
	pub mass: f32,
	pub radius: f32,
	pub pos: Vector2<f32>,
	pub vel: Vector2<f32>,
	acc: Cell<Vector2<f32>>,
}

impl Physics {
	pub fn at_rest(mass: f32, radius: f32) -> Self {
		Self {
			mass,
			radius,
			pos: vec2(0., 0.),
			vel: vec2(0., 0.),
			acc: vec2(0., 0.).into(),
		}
	}

	pub fn set_acc(&self, acc: Vector2<f32>) {
		self.acc.set(acc);
	}

	pub fn update(&mut self, delta: f32) {
		self.pos += delta * self.vel;
		self.vel += delta * self.acc.get();
	}
}

pub fn gravity(object: &Physics, universe: &Grid<Box<dyn Celestial>>) -> Vector2<f32> {
	const RADIUS: f32 = 10_000.;
	const GRAV_CONST: f32 = 100.0;

	let mut acc = vec2(0., 0.);

	let p1 = object.pos;
	for (r, boxed) in universe.query_with_dist(p1.into(), RADIUS) {
		let obj = &**boxed;
		let m2 = obj.phys().mass;
		let p2 = obj.phys().pos;

		if r > 0. {
			acc += (p2 - p1).normalize_to(GRAV_CONST * m2 / (r * r));
		}
	}

	acc
}
