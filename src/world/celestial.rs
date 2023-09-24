use super::*;

pub trait Celestial: Griddable + GameObject<Scene = World, Action = Action> + 'static {
	fn phys(&self) -> &Physics;
	fn phys_mut(&mut self) -> &mut Physics;

	fn boxed(self) -> Box<dyn Celestial>
	where
		Self: Sized,
	{
		Box::new(self) as Box<dyn Celestial>
	}
}

impl<C: Celestial> Griddable for C {
	fn pos(&self) -> (f32, f32) {
		self.phys().pos.into()
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
		(**self).phys().pos.into()
	}
}
