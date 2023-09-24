use super::*;

pub fn apply_collisions(universe: &mut Universe) {
	const MAX_DIAMETER: f32 = 150.;
	universe.maintain();
	universe.apply_to_pairs(MAX_DIAMETER, elastic_collision);
}

pub fn elastic_collision(body_1: &mut Box<dyn Celestial>, body_2: &mut Box<dyn Celestial>) {
	//from https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects

	let a = body_1.phys_mut();
	let b = body_2.phys_mut();

	let x1 = a.pos;
	let x2 = b.pos;

	let v1 = a.vel;
	let v2 = b.vel;

	let m1 = a.mass;
	let m2 = b.mass;

	let diff = x1 - x2;
	let dist = diff.magnitude();

	if dist < (a.diameter + b.diameter) / 2. {

		let common_factor = 2. * (v1 - v2).dot(diff) / ((m1 + m2) * (diff).dot(diff)) * diff;

		a.vel -= b.mass * common_factor;
		b.vel += a.mass * common_factor;

		//Ensure they are separated
		a.pos = b.pos + (diff * 1.01);		

	}
}
