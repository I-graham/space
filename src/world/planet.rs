use super::*;

pub struct Planet {
	phys: Physics,
}

impl Planet {
	pub fn random() -> Self {
		let phys = Physics::rest(50., 48.)
			.at(rand_in2d(-500., 500.))
			.heading(rand_in2d(-50., 50.))
			.spinning(rand_in(-5., 5.));

		Self { phys }
	}
}

impl GameObject for Planet {
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
		self.phys.instance(Texture::Neptune, external).into()
	}
}

impl Celestial for Planet {
	fn phys(&self) -> &Physics {
		&self.phys
	}

	fn phys_mut(&mut self) -> &mut Physics {
		&mut self.phys
	}
}
