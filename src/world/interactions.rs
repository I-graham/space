use super::*;
use cgmath::*;

pub fn gravitational_acceleration(
	object: &impl Celestial,
	universe: &Grid<Box<dyn Celestial>>,
) -> Vector2<f32> {
	const RADIUS: f32 = 10_000.;
	const GRAV_CONST: f32 = 100.0;

	let mut acc = vec2(0., 0.);

	let p1 = object.coords();
	for (r, boxed) in universe.query_with_dist(p1.into(), RADIUS) {
		let obj = &**boxed;
		let m2 = obj.mass();
		let p2 = obj.coords();

		if r > 0. {
			acc += (p2 - p1).normalize_to(GRAV_CONST * m2 / (r * r));
		}
	}

	acc
}
