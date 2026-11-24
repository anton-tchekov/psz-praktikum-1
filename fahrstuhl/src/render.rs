use macroquad::prelude::*;
use crate::constants::*;
use crate::cube::*;
use crate::floor::*;
use crate::cabin::*;

const BG_COLOR: Color = Color::new(0.02, 0.12, 0.18, 1.0);
const DOOR_COLOR: Color = Color::new(0.04, 0.24, 0.36, 1.0);
const PL_COLOR: Color = Color::new(0.02, 0.18, 0.12, 1.0);
const PD_COLOR: Color = Color::new(0.04, 0.36, 0.24, 1.0);

pub fn draw_inner_outline(pos: Vec3, h0: f32, h1: f32)
{
	const XS: f32 = WALL + EPS;
	const XDS: f32 = DOOR_X + EPS;
	const XDE: f32 = WIDTH - DOOR_X - EPS;
	const XE: f32 = WIDTH - WALL - EPS;
	const YS: f32 = WALL + EPS;
	const YE: f32 = DEPTH - WALL - EPS;

	let p =
	[
		vec2(XS, YE),
		vec2(XE, YE),
		vec2(XE, YS),
		vec2(XDE, YS),
		vec2(XDE, -EPS),
		vec2(XDS, -EPS),
		vec2(XDS, YS),
		vec2(XS, YS),
		vec2(XS, YE)
	];

	for i in 1..p.len()
	{
		let a = p[i - 1];
		let b = p[i];
		draw_line_3d(pos + vec3(a.x, h0, a.y), pos + vec3(b.x, h0, b.y), WHITE);
		draw_line_3d(pos + vec3(a.x, h1, a.y), pos + vec3(b.x, h1, b.y), WHITE);
	}

	for a in p.iter().take(p.len() - 1)
	{
		draw_line_3d(pos + vec3(a.x, h0, a.y), pos + vec3(a.x, h1, a.y), WHITE);
	}
}

pub fn draw_door(pos: Vec3, door: f32, color: Color)
{
	let dw = door * (WIDTH - 2.0 * DOOR_X - GAP) / 2.0;

	draw_cube_with_outline(pos + vec3(DOOR_X, GAP / 2.0, WALL / 4.0),
		vec3(dw, HEIGHT - 2.0 * WALL - GAP, WALL / 2.0), color);

	draw_cube_with_outline(pos + vec3(WIDTH - DOOR_X - dw, GAP / 2.0, WALL / 4.0),
		vec3(dw, HEIGHT - 2.0 * WALL - GAP, WALL / 2.0), color);
}

pub fn draw_elevator(pos: Vec3, door: f32)
{
	// Bottom Base
	draw_cube_colored(pos + vec3(0.0, -WALL, 0.0), vec3(WIDTH, WALL, DEPTH), BG_COLOR);

	// Top Plate
	draw_cube_colored(pos + vec3(0.0, HEIGHT - 2.0 * WALL, 0.0), vec3(WIDTH, WALL, DEPTH), BG_COLOR);

	// Back Plate
	draw_cube_colored(pos + vec3(0.0, 0.0, DEPTH - WALL),
		vec3(WIDTH, HEIGHT - 2.0 * WALL, WALL), BG_COLOR);

	// Front Segments
	draw_cube_colored(pos + vec3(0.0, 0.0, 0.0),
		vec3(DOOR_X, HEIGHT - 2.0 * WALL, WALL), BG_COLOR);

	draw_cube_colored(pos + vec3(WIDTH - DOOR_X, 0.0, 0.0),
		vec3(DOOR_X, HEIGHT - 2.0 * WALL, WALL), BG_COLOR);

	// Left and Right Side
	draw_cube_colored(pos + vec3(0.0, 0.0, WALL),
		vec3(WALL, HEIGHT - 2.0 * WALL, DEPTH - 2.0 * WALL), BG_COLOR);

	draw_cube_colored(pos + vec3(WIDTH - WALL, 0.0, WALL),
		vec3(WALL, HEIGHT - 2.0 * WALL, DEPTH - 2.0 * WALL), BG_COLOR);

	// Outline Outer
	draw_cube_outline(pos + vec3(0.0, -WALL, 0.0), vec3(WIDTH, HEIGHT, DEPTH));

	// Outline Inner
	draw_inner_outline(pos, EPS, HEIGHT - 2.0 * WALL - EPS);

	draw_door(pos, door, DOOR_COLOR);
}

pub fn draw_indicator(pos: Vec3, color: Color, size: f32)
{
	let vertices = vec![
		Vertex::new2(vec3(pos.x - size, pos.y, pos.z), vec2(0.0, 0.0), color),
		Vertex::new2(vec3(pos.x + size, pos.y, pos.z), vec2(0.0, 0.0), color),
		Vertex::new2(vec3(pos.x, pos.y + size, pos.z), vec2(0.0, 0.0), color)
	];

	let indices = vec![ 0, 1, 2 ];
	let mesh = Mesh { vertices, indices, texture: None };
	draw_mesh(&mesh);
}

pub fn draw_digit(pos: Vec3, color: Color, digit: usize, size: f32)
{
	const DIGITS: [u8; 10] =
	[
		0x3F, 0x06, 0x5B, 0x4F, 0x66, 0x6D, 0x7D, 0x07, 0x7F, 0x6F
	];

	let segments = DIGITS[digit];

	let q = size;
	let f = q / 6.0;

	let v = vec3(f, q - 2.0 * f, f);
	let h = vec3(q - 2.0 * f, f, f);

	if segments & (1 << 3) != 0 { draw_cube_colored(pos + vec3(f, 0.0, 0.0), h, color); }
	if segments & (1 << 4) != 0 { draw_cube_colored(pos + vec3(q - f, f, 0.0), v, color); }
	if segments & (1 << 5) != 0 { draw_cube_colored(pos + vec3(q - f, q, 0.0), v, color); }
	if segments & (1 << 0) != 0 { draw_cube_colored(pos + vec3(f, 2.0 * (q - f), 0.0), h, color); }
	if segments & (1 << 1) != 0 { draw_cube_colored(pos + vec3(0.0, q, 0.0), v, color); }
	if segments & (1 << 2) != 0 { draw_cube_colored(pos + vec3(0.0, f, 0.0), v, color); }
	if segments & (1 << 6) != 0 { draw_cube_colored(pos + vec3(f, q - f, 0.0), h, color); }
}

pub fn draw_scene(cabins: &[Cabin], floors: &[Floor])
{
	for (i, floor) in floors.iter().enumerate().take(NUM_FLOORS)
	{
		let y = i as f32 * FLOOR_HEIGHT - WALL;

		draw_cube_with_outline(vec3(0.0, y, 0.0),
			vec3(FLOOR_W, WALL, FLOOR_D), PL_COLOR);

		draw_cube_with_outline(vec3(0.0, y + WALL + 2.0 * EPS, FLOOR_D - WALL),
			vec3(WALL + DOOR_X, HEIGHT, WALL), PL_COLOR);
		draw_cube_with_outline(vec3(FLOOR_W - WALL - DOOR_X, y + WALL + 2.0 * EPS, FLOOR_D - WALL),
			vec3(WALL + DOOR_X, HEIGHT, WALL), PL_COLOR);

		for j in 0..NUM_CABINS - 1
		{
			draw_cube_with_outline(vec3(WALL + WIDTH - DOOR_X + j as f32 * CABIN_STEP, y + WALL + 2.0 * EPS, FLOOR_D - WALL),
				vec3(CABIN_STEP - WIDTH + 2.0 * DOOR_X, HEIGHT, WALL), PL_COLOR);
		}

		for j in 0..NUM_CABINS
		{
			draw_door(vec3(j as f32 * CABIN_STEP + WALL, y + WALL, FLOOR_D - WALL), floor.doors[j], PD_COLOR);
		}

		if i != NUM_FLOORS - 1
		{
			draw_indicator(vec3(1.45, y + 1.6, FLOOR_D - 0.06), if floor.indicator_up_on { GREEN } else { GRAY }, 0.2);
		}

		if i != 0
		{
			draw_indicator(vec3(1.45, y + 0.95, FLOOR_D - 0.06), if floor.indicator_down_on { RED } else { GRAY }, -0.2);
		}

		draw_digit(vec3(1.3, y + 1.0, FLOOR_D - 0.1), WHITE, i + 1, 0.3);
	}

	for (i, cabin) in cabins.iter().enumerate()
	{
		draw_elevator(vec3(i as f32 * CABIN_STEP + WALL, cabin.y, FLOOR_D + GAP), cabin.door);
	}
}
