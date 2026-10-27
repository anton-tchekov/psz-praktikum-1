use macroquad::prelude::*;

const W: usize = 10;
const H: usize = 10;
const N: usize = W * H;
const TILE_SIZE: f32 = 32.0;
const OX: f32 = 10.0;
const OY: f32 = 50.0;

#[macroquad::main("Minesweeper")]
async fn main()
{
	let texture: Texture2D = load_texture("minesweeper.png").await.unwrap();

	let mut tiles: [u8; N] = [0u8; N];

	for y in 0..H
	{
		for x in 0..W
		{
			tiles[y * W + x] = rand::gen_range(0, 12);
		}
	}

	loop
	{
		clear_background(BLACK);
		draw_text("PSZ-01: Minesweeper", 10.0, 25.0, 30.0, WHITE);

		for y in 0..H
		{
			for x in 0..W
			{
				let px = x as f32 * TILE_SIZE + OX;
				let py = y as f32 * TILE_SIZE + OY;

				let tile_id = tiles[y * W + x];
				let tx = (tile_id / 4) as f32 * TILE_SIZE;
				let ty = (tile_id % 4) as f32 * TILE_SIZE;

				draw_texture_ex(&texture, px, py, WHITE,
					DrawTextureParams {
						dest_size: None,
						source: Some(Rect::new(tx, ty, TILE_SIZE, TILE_SIZE)),
						rotation: 0.0,
						flip_x: false,
						flip_y: false,
						pivot: None
					});
			}
		}


		next_frame().await
	}
}
