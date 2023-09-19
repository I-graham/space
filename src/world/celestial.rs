use super::*;
use cgmath::*;

pub trait Celestial: Griddable + GameObject<Scene = World, Action = Action> + 'static {
	fn coords(&self) -> Vector2<f32>;
	fn mass(&self) -> f32;
	fn radius(&self) -> f32;

	fn boxed(self) -> Box<dyn Celestial>
	where
		Self: Sized,
	{
		Box::new(self) as Box<dyn Celestial>
	}
}

impl<C: Celestial> Griddable for C {
	fn pos(&self) -> (f32, f32) {
		self.coords().into()
	}
}

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
		(**self).coords().into()
	}
}
