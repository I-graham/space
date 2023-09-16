mod texture;

pub use texture::*;

use super::*;
use crate::window::*;
use eng::*;
use tracing::trace_span;

pub struct World {}

impl Root for World {
	type Texture = Texture;

	fn init() -> Self {
		Self {}
	}
}

impl GameObject for World {
	type Scene = ();
	type Action = ();

	fn plan(&self, _: &(), external: &External, messenger: &Sender<Dispatch>) {
		let span = trace_span!("Planning");
		let _guard = span.enter();
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		let span = trace_span!("Updating");
		let _guard = span.enter();

		None
	}

	fn render(&self, external: &External, out: &mut Vec<Instance>) {
		let span = trace_span!("Rendering");
		let _guard = span.enter();
	}

	fn cleanup(&mut self) {
		let span = trace_span!("Debug info");
		let _guard = span.enter();
	}
}
