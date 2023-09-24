mod action;
mod background;
mod celestial;
mod interface;
mod physics;
mod planet;
mod ship;
mod sun;
mod texture;

pub use action::*;
pub use background::*;
pub use celestial::*;
pub use interface::*;
pub use physics::*;
pub use planet::*;
pub use ship::*;
pub use sun::*;
pub use texture::*;

use super::*;
use crate::window::*;
use eng::*;
use tracing::trace_span;

type Universe = Grid<Box<dyn Celestial>>;

pub struct World {
	universe: Universe,
	background: Background,
	interface: Interface,
}

impl Root for World {
	type Texture = Texture;

	fn init(external: &External) -> Self {
		use std::iter::repeat_with;
		let universe = repeat_with(|| Sun::random().boxed())
			.take(10)
			.chain(repeat_with(|| Planet::random().boxed()).take(5))
			.chain(repeat_with(|| Ship::random().boxed()).take(5));

		Self {
			universe: Universe::from_iter(1000.0, universe),
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

		self.interface.plan(self, external, messenger);
		self.universe.plan(self, external, messenger);
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		let span = trace_span!("Updating");
		let _guard = span.enter();

		self.interface.update(external, messenger);
		self.universe.update(external, messenger);
		apply_collisions(&mut self.universe);

		None
	}

	fn render(&self, win: &mut Window) {
		let span = trace_span!("Rendering");
		let _guard = span.enter();

		self.interface.render(win);
		self.background.render(win);
		self.universe.render(win);
	}

	fn cleanup(&mut self) {
		let span = trace_span!("Debug info");
		let _guard = span.enter();

		self.universe.cleanup();
	}
}
