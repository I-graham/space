use super::*;

pub struct Background;

impl Background {
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
		const STARS: usize = 3000;

		let cache = "Stars";
		if !win.is_cached(cache) {
			let mut instances = vec![];

			instances.push(
				Instance {
					color_tint: (0., 0., 0., 1.).into(),
					screen_relative: GLbool::True,
					..win.inputs().instance(Texture::Flat)
				}
				.scale(2.),
			);

			for _ in 0..STARS {
				let star = utils::rand_in2d(-2000., 2000.);
				instances.push(
					Instance {
						rotation: 45f32.into(),
						position: star.into(),
						..win.inputs().instance(Texture::Flat)
					}
					.scale(2.),
				);
			}

			win.cache(cache, &instances);
		}

		win.draw_cached(cache, (0., 0.), 1.0);
	}
}
