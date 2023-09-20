use super::*;

pub struct Planet {
	phys: Physics,
}

impl Planet {
	pub fn new() -> Self {
		Self {
			phys: Physics::at_rest(1000., 64.),
		}
	}
}

impl GameObject for Planet {
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
			..external.instance(Texture::Sun)
		})
	}
}

impl Celestial for Planet {
	fn phys(&self) -> &Physics {
		&self.phys
	}
}
