use macroquad::prelude::*;
use crate::cube::*;

pub struct Player
{
	pos: Vec3,
	rot: f32,
	walking: bool,
	skin_id: usize
}

const fn t(x: i32, y: i32, w: i32, h: i32) -> Rect
{
	const TW: f32 = 64.0;
	Rect::new(x as f32 / TW, y as f32 / TW, w as f32 / TW, h as f32 / TW)
}

const HEAD_TEX: [Rect; 6] =
[
	t(8, 8, 8, 8), t(24, 8, 8, 8),
	t(16, 8, 8, 8), t(0, 8, 8, 8),
	t(8, 0, 8, 8), t(16, 0, 8, 8)
];

const RIGHT_LEG_TEX: [Rect; 6] =
[
	t(4, 20, 4, 12), t(12, 20, 4, 12),
	t(0, 20, 4, 12), t(8, 20, 4, 12),
	t(4, 16, 4, 4),  t(8, 16, 4, 4)
];

const LEFT_LEG_TEX: [Rect; 6] =
[
	t(20, 52, 4, 12), t(28, 52, 4, 12),
	t(16, 52, 4, 12), t(24, 52, 4, 12),
	t(20, 48, 4, 4),  t(24, 48, 4, 4)
];

const BODY_TEX: [Rect; 6] =
[
	t(20, 20, 8, 12), t(32, 20, 8, 12),
	t(16, 20, 4, 12), t(28, 20, 4, 12),
	t(20, 16, 8, 4),  t(28, 16, 8, 4)
];

const LEFT_ARM_TEX: [Rect; 6] =
[
	t(36, 52, 4, 12), t(44, 52, 4, 12),
	t(32, 52, 4, 12), t(40, 52, 4, 12),
	t(36, 48, 4, 4),  t(40, 48, 4, 4)
];

const RIGHT_ARM_TEX: [Rect; 6] =
[
	t(44, 20, 4, 12), t(52, 20, 4, 12),
	t(40, 20, 4, 12), t(48, 20, 4, 12),
	t(44, 16, 4, 4),  t(48, 16, 4, 4)
];

const LEFT_ARM_TEX_SLIM: [Rect; 6] =
[
	t(36, 52, 3, 12), t(43, 52, 3, 12),
	t(32, 52, 4, 12), t(39, 52, 4, 12),
	t(36, 48, 3, 4),  t(39, 48, 3, 4)
];

const RIGHT_ARM_TEX_SLIM: [Rect; 6] =
[
	t(44, 20, 3, 12), t(51, 20, 3, 12),
	t(40, 20, 4, 12), t(47, 20, 4, 12),
	t(44, 16, 3, 4),  t(47, 16, 3, 4)
];

pub fn draw_player(pos: Vec3, scale: f32, arm_thin: bool, texture: &Texture2D)
{
	let head_size = 8.0 * scale;
	let arm_wide = if arm_thin { 3.0 } else { 4.0 } * scale;
	let arm_thick = 4.0 * scale;
	let arm_height = 12.0 * scale;
	let leg_width = 4.0 * scale;
	let leg_height = 12.0 * scale;
	let body_width = 8.0 * scale;
	let body_thick = 4.0 * scale;
	let body_height = 12.0 * scale;

	// Legs
	draw_cube_textured(pos + vec3(-leg_width, 0.0, -leg_width / 2.0),
		vec3(leg_width, leg_height, leg_width),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, LEFT_LEG_TEX);

	draw_cube_textured(pos + vec3(0.0, 0.0, -leg_width / 2.0),
		vec3(leg_width, leg_height, leg_width),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, RIGHT_LEG_TEX);

	// Arms
	draw_cube_textured(pos + vec3(-leg_width - arm_wide, leg_height, -arm_thick / 2.0),
		vec3(arm_wide, arm_height, arm_thick),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, if arm_thin { RIGHT_ARM_TEX_SLIM } else { RIGHT_ARM_TEX });

	draw_cube_textured(pos + vec3(leg_width, leg_height, -arm_thick / 2.0),
		vec3(arm_wide, arm_height, arm_thick),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, if arm_thin { LEFT_ARM_TEX_SLIM } else { LEFT_ARM_TEX });

	// Body
	draw_cube_textured(pos + vec3(-body_width / 2.0, leg_height, -body_thick / 2.0),
		vec3(body_width, body_height, body_thick),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, BODY_TEX);

	// Head
	draw_cube_textured(pos + vec3(-head_size / 2.0, leg_height + body_height, -head_size / 2.0),
		vec3(head_size, head_size, head_size),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, HEAD_TEX);
}
