use utils::ui::Parent;

use super::*;

pub struct Interface {
	ui: Parent,
}

impl Interface {
	pub fn new(external: &External) -> Self {
		Self {
			ui: Parent::screen(external),
		}
	}
}

impl GameObject for Interface {
	type Scene = World;
	type Action = Action;

	fn plan(&self, _scene: &Self::Scene, external: &External, messenger: &Sender<Dispatch>) {
		self.ui.plan(&self.ui, external, messenger)
	}

	fn update(&mut self, external: &External, messenger: &Messenger) -> Option<Self::Action> {
		self.ui.update(external, messenger);
		None
	}

	fn render(&self, win: &mut Window) {
		self.ui.render(win)
	}
}
