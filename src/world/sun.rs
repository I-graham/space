use cgmath::*;

use super::*;

pub struct Sun {
	pos: Vector2<f32>,
}

impl GameObject for Sun {
	type Scene = World;
	type Action = Action;

	fn plan(&self, _scene: &Self::Scene, _external: &External, _messenger: &Sender<Dispatch>) {}

	fn update(&mut self, _external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		None
	}

	fn instance(&self, external: &External) -> Option<Instance> {
		Some(Instance {
			..external.instance(Texture::Sun)
		})
	}
}

impl Sun {
	pub fn new() -> Self {
		Self { pos: vec2(0., 0.) }
	}
}

impl Griddable for Sun {
	fn pos(&self) -> (f32, f32) {
		self.pos.into()
	}
}
