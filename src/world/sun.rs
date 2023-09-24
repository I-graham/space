use super::*;

pub struct Sun {
	phys: Physics,
}

impl Sun {
	pub fn random() -> Self {
		let phys = Physics::rest(5000., 128.)
			.at(rand_in2d(-600., 600.))
			.heading(rand_in2d(-5., 5.))
			.spinning(rand_in(-5., 5.));

		Self { phys }
	}
}

impl GameObject for Sun {
	type Scene = World;
	type Action = Action;

	fn plan(&self, scene: &Self::Scene, _external: &External, _messenger: &Sender<Dispatch>) {
		self.phys.apply_force(gravity(&self.phys, &scene.universe))
	}

	fn update(&mut self, external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		self.phys.update(external.delta);

		None
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		self.phys.instance(Texture::Sun, external).into()
	}
}

impl Celestial for Sun {
	fn phys(&self) -> &Physics {
		&self.phys
	}

	fn phys_mut(&mut self) -> &mut Physics {
		&mut self.phys
	}
}
