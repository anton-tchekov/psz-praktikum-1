use macroquad::prelude::*;

pub struct Skin
{
	pub slim: bool,
	pub texture: Texture2D
}

pub async fn load_skins() -> Vec<Skin>
{
	let filenames =
	[
		("assets/alex.png", true),
		("assets/black_boy.png", false),
		("assets/blueberry_girl.png", true),
		("assets/kiki.png", true),
		("assets/pink.png", true),
		("assets/asuka.png", true),
		("assets/black_hair.png", true),
		("assets/cap.png", false),
		("assets/luffy.png", false),
		("assets/miku.png", false),
		("assets/steve.png", false),
	];

	let mut skins = Vec::with_capacity(filenames.len());
	for (filename, slim) in filenames
	{
		let texture = load_texture(filename).await.unwrap();
		texture.set_filter(FilterMode::Nearest);
		skins.push(Skin { slim, texture });
	}

	skins
}
