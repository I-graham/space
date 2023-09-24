use super::*;

pub fn gravity(object: &Physics, universe: &Universe) -> Vector2<f32> {
	const SEARCH_RADIUS: f32 = f32::INFINITY;
	const GRAVITATIONAL_CONSTANT: f32 = 500.0;

	let mut acc = vec2(0., 0.);

	let p1 = object.pos;
	let m1 = object.mass;
	for (r, boxed) in universe.query_with_dist(p1.into(), SEARCH_RADIUS) {
		let obj = &**boxed;
		let m2 = obj.phys().mass;
		let p2 = obj.phys().pos;

		if r > 0. {
			acc += (p2 - p1).normalize_to(GRAVITATIONAL_CONSTANT * m1 * m2 / (r * r + 5.));
		}
	}

	acc
}
