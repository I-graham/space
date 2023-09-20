use super::*;

pub struct Ship {
	phys: Physics,
}

impl Ship {
	pub fn new() -> Self {
		let mut phys = Physics::at_rest(10., 16.);
		phys.pos = cgmath::vec2(50., 50.);
		phys.vel = cgmath::vec2(-5., 20.);

		Self { phys }
	}
}

impl GameObject for Ship {
	type Scene = World;
	type Action = Action;

	fn plan(&self, scene: &Self::Scene, _external: &External, _messenger: &Sender<Dispatch>) {
		self.phys.set_acc(gravity(&self.phys, &scene.celestials))
	}

	fn update(&mut self, external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		self.phys.update(external.delta);

		None
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		Some(Instance {
			position: self.phys.pos.into(),
			..external
				.instance(Texture::Ship)
				.nth_frame(0, Texture::Ship.frame_count())
		})
	}
}

impl Celestial for Ship {
	fn phys(&self) -> &Physics {
		&self.phys
	}
}
