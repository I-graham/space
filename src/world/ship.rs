use super::*;

pub struct Ship {
	phys: Physics,
}

impl Ship {
	pub fn random() -> Self {
		let phys = Physics::rest(10., 32.)
			.at(rand_in2d(-500., 500.))
			.heading(rand_in2d(-50., 50.))
			.spinning(rand_in(-1., 1.));

		Self { phys }
	}
}

impl GameObject for Ship {
	type Scene = World;
	type Action = Action;

	fn plan(&self, scene: &Self::Scene, _external: &External, _messenger: &Sender<Dispatch>) {
		self.phys
			.apply_force(gravity(&self.phys, &scene.universe))
	}

	fn update(&mut self, external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		self.phys.update(external.delta);

		None
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		self.phys
			.instance(Texture::Ship, external)
			.nth_frame(0, Texture::Ship.frame_count())
			.into()
	}
}

impl Celestial for Ship {
	fn phys(&self) -> &Physics {
		&self.phys
	}

	fn phys_mut(&mut self) -> &mut Physics {
		&mut self.phys
	}
}
