mod action;
mod background;
mod sun;
mod texture;

pub use action::*;
pub use background::*;
pub use sun::*;
pub use texture::*;

use super::*;
use crate::window::*;
use eng::*;
use tracing::trace_span;

pub struct World {
	celestials: Grid<Box<dyn Celestial>>,
	background: Background,
}

impl Root for World {
	type Texture = Texture;

	fn init() -> Self {
		let celestials = Grid::from_iter(1000.0, [Sun::new().boxed()].into_iter());

		Self {
			celestials,
			background: Background::new(),
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
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		let span = trace_span!("Updating");
		let _guard = span.enter();
		self.celestials.update(external, messenger);
		None
	}

	fn render(&self, win: &mut Window) {
		let span = trace_span!("Rendering");
		let _guard = span.enter();
		self.background.render(win);
		self.celestials.render(win);
	}

	fn cleanup(&mut self) {
		let span = trace_span!("Debug info");
		let _guard = span.enter();
		self.celestials.cleanup();
	}
}

trait Celestial: Griddable + GameObject<Scene = World, Action = Action> + 'static {
	fn boxed(self) -> Box<dyn Celestial>
	where
		Self: Sized,
	{
		Box::new(self) as Box<dyn Celestial>
	}
}
impl<T: Griddable + GameObject<Scene = World, Action = Action> + 'static> Celestial for T {}

impl GameObject for Box<dyn Celestial> {
	type Scene = World;
	type Action = Action;

	fn plan(&self, scene: &Self::Scene, external: &External, messenger: &Sender<Dispatch>) {
		(**self).plan(scene, external, messenger)
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		(**self).update(external, messenger)
	}

	fn render(&self, win: &mut Window) {
		(**self).render(win)
	}

	fn cleanup(&mut self) {
		(**self).cleanup()
	}
}

impl Griddable for Box<dyn Celestial> {
	fn alive(&self) -> bool {
		(**self).alive()
	}

	fn pos(&self) -> (f32, f32) {
		(**self).pos()
	}
}
