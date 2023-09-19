mod action;
mod background;
mod celestial;
mod interactions;
mod interface;
mod ship;
mod sun;
mod texture;

pub use action::*;
pub use background::*;
pub use celestial::*;
pub use interactions::*;
pub use interface::*;
pub use ship::*;
pub use sun::*;
pub use texture::*;

use super::*;
use crate::window::*;
use eng::*;
use tracing::trace_span;

pub struct World {
	celestials: Grid<Box<dyn Celestial>>,
	background: Background,
	interface: Interface,
}

impl Root for World {
	type Texture = Texture;

	fn init(external: &External) -> Self {
		let universe = [Sun::new().boxed(), Ship::new().boxed()];

		Self {
			celestials: Grid::from_iter(1000.0, universe.into_iter()),
			background: Background::new(),
			interface: Interface::new(external),
		}
	}
}

impl GameObject for World {
	type Scene = ();
	type Action = ();

	fn plan(&self, _: &(), external: &External, messenger: &Sender<Dispatch>) {
		let span = trace_span!("Planning");
		let _guard = span.enter();
		self.celestials.plan(self, external, messenger);
		self.interface.plan(self, external, messenger);
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		let span = trace_span!("Updating");
		let _guard = span.enter();
		self.celestials.update(external, messenger);
		self.interface.update(external, messenger);
		None
	}

	fn render(&self, win: &mut Window) {
		let span = trace_span!("Rendering");
		let _guard = span.enter();
		self.background.render(win);
		self.celestials.render(win);
		self.interface.render(win);
	}

	fn cleanup(&mut self) {
		let span = trace_span!("Debug info");
		let _guard = span.enter();
		self.celestials.cleanup();
	}
}
