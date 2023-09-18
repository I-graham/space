use super::*;

pub struct Background;

impl Background {
	const STARS: usize = 500;

	pub fn new() -> Self {
		Self
	}
}

impl GameObject for Background {
	type Scene = World;
	type Action = ();

	fn plan(&self, _scene: &Self::Scene, _external: &External, _messenger: &Sender<Dispatch>) {}

	fn update(&mut self, _external: &External, _messenger: &Messenger) -> Option<Self::Action> {
		None
	}

	fn render(&self, win: &mut Window) {
		let cache = "Stars";
		if !win.is_cached(cache) {
			let mut instances = vec![];

			instances.push(Instance {
				color_tint: (0., 0., 0., 1.).into(),
				scale: (2., 2.).into(),
				screen_relative: GLbool::True,
				..win.inputs().instance(Texture::Flat)
			});

			for _ in 0..Self::STARS {
				let star = utils::rand_in2d(-1000., 1000.);
				instances.push(Instance {
					rotation: 45.0.into(),
					position: star.into(),
					color_tint: (0., 0., 0., 1.).into(),
					..win.inputs().instance(Texture::Flat)
				});
			}

			win.cache(cache, &instances);
		}

		win.draw_cached(cache, (0., 0.), 1.0);
	}
}
