use macroquad::prelude::*;

const NUM_CABINS: usize = 3;
const NUM_FLOORS: usize = 4;

const BG_COLOR: Color = Color::new(0.02, 0.12, 0.18, 1.0);
const DOOR_COLOR: Color = Color::new(0.04, 0.24, 0.36, 1.0);

const PL_COLOR: Color = Color::new(0.02, 0.18, 0.12, 1.0);
const PD_COLOR: Color = Color::new(0.04, 0.36, 0.24, 1.0);

const WALL: f32 = 0.05;
const HEIGHT: f32 = 2.0;
const WIDTH: f32 = 1.0;
const DEPTH: f32 = 1.2;
const EPS: f32 = 0.003;
const GAP: f32 = 0.02;

const DOOR_SPEED: f32 = 2.5;
const CABIN_SPEED: f32 = 2.5;

const CABIN_STEP: f32 = 2.0;
const FLOOR_HEIGHT: f32 = 3.0;

const FLOOR_D: f32 = 3.0;

const DOOR_X: f32 = WIDTH / 6.0;

const MOVE_SPEED: f32 = 3.0;
const MOUSE_SENSITIVITY: f32 = 0.001;

#[derive(Copy, Clone, PartialEq)]
enum CabinState
{
	DoorClosing,
	Moving,
	DoorOpening,
	Stopped
}

#[derive(Copy, Clone, PartialEq)]
enum PassengerState
{
	Idle,
	Entering,
	Choosing,
	Riding,
	Leaving
}

#[derive(Copy, Clone)]
struct Cabin
{
	y: f32,
	door: f32,
	from: usize,
	target: usize,
	state: CabinState
}

impl Cabin
{
	fn new() -> Self
	{
		Self { y: 0.0, door: 0.0, from: 0, target: 0, state: CabinState::Stopped }
	}
}

#[derive(Copy, Clone)]
struct Floor
{
	doors: [f32; NUM_CABINS]
}

impl Floor
{
	fn new() -> Self
	{
		Self { doors: [1.0; NUM_CABINS] }
	}
}

struct CameraParams
{
	pos: Vec3,
	yaw: f32,
	pitch: f32,
	last_mouse: Vec2
}

impl CameraParams
{
	fn new(pos: Vec3) -> Self
	{
		Self
		{
			pos,
			yaw: 0.0,
			pitch: 0.0,
			last_mouse: mouse_position().into()
		}
	}

	fn update(&mut self)
	{
		let dt = get_frame_time();

		let mouse: Vec2 = mouse_position().into();
		if is_mouse_button_pressed(MouseButton::Left)
		{
			self.last_mouse = mouse;
		}

		if is_mouse_button_down(MouseButton::Left)
		{
			let delta = mouse - self.last_mouse;
			self.yaw += delta.x * MOUSE_SENSITIVITY;
			self.pitch = (self.pitch - delta.y * MOUSE_SENSITIVITY).clamp(-1.5, 1.5);
		}

		self.last_mouse = mouse;

		let forward = vec3(self.yaw.sin(), 0.0, -self.yaw.cos());
		let right = vec3(self.yaw.cos(), 0.0, self.yaw.sin());

		let mut movement = Vec3::ZERO;
		if is_key_down(KeyCode::W) { movement += forward; }
		if is_key_down(KeyCode::S) { movement -= forward; }
		if is_key_down(KeyCode::D) { movement += right; }
		if is_key_down(KeyCode::A) { movement -= right; }
		if is_key_down(KeyCode::Space) { movement.y += 1.0; }
		if is_key_down(KeyCode::LeftShift) { movement.y -= 1.0; }

		if movement.length_squared() > 0.0
		{
			self.pos += movement.normalize() * MOVE_SPEED * dt;
		}

		let look_dir = vec3(
			self.yaw.sin() * self.pitch.cos(),
			self.pitch.sin(),
			-self.yaw.cos() * self.pitch.cos(),
		);

		set_camera(&Camera3D
			{
				position: self.pos,
				target: self.pos + look_dir,
				up: vec3(0.0, 1.0, 0.0),
				..Default::default()
			});
	}
}

fn draw_my_cube(a: Vec3, b: Vec3)
{
	let a = a + b * 0.5;
	draw_cube(a, b, None, BG_COLOR);
}

fn draw_tex_cube(pos: Vec3, size: Vec3, rot_point: Vec3, rotation: Vec3,
	atlas: &Texture2D, rect: [Rect; 6])
{
	let rotation_matrix = Mat4::from_rotation_z(rotation.z) *
		Mat4::from_rotation_y(rotation.y) * Mat4::from_rotation_x(rotation.x);

	let transform = Mat4::from_translation(rot_point) *
		rotation_matrix * Mat4::from_translation(-rot_point);

	let (x0, y0, z0) = (pos.x, pos.y, pos.z);
	let (x1, y1, z1) = (pos.x + size.x, pos.y + size.y, pos.z + size.z);

	let faces: [[Vec3; 4]; 6] =
	[
		[vec3(x0, y1, z1), vec3(x0, y0, z1), vec3(x1, y0, z1), vec3(x1, y1, z1)], // Front (+Z)
		[vec3(x1, y1, z0), vec3(x1, y0, z0), vec3(x0, y0, z0), vec3(x0, y1, z0)], // Back (-Z)
		[vec3(x1, y1, z1), vec3(x1, y0, z1), vec3(x1, y0, z0), vec3(x1, y1, z0)], // Right (+X)
		[vec3(x0, y1, z0), vec3(x0, y0, z0), vec3(x0, y0, z1), vec3(x0, y1, z1)], // Left (-X)
		[vec3(x0, y1, z0), vec3(x0, y1, z1), vec3(x1, y1, z1), vec3(x1, y1, z0)], // Top (+Y)
		[vec3(x0, y0, z1), vec3(x0, y0, z0), vec3(x1, y0, z0), vec3(x1, y0, z1)]  // Bottom (-Y)
	];

	let mut vertices = Vec::with_capacity(24);
	let mut indices = Vec::with_capacity(36);
	for (face_index, corners) in faces.iter().enumerate()
	{
		let face_rect = rect[face_index];
		let uvs =
		[
			vec2(face_rect.x,               face_rect.y),
			vec2(face_rect.x,               face_rect.y + face_rect.h),
			vec2(face_rect.x + face_rect.w, face_rect.y + face_rect.h),
			vec2(face_rect.x + face_rect.w, face_rect.y)
		];

		let base = (face_index * 4) as u16;
		for i in 0..4
		{
			let p = transform * vec4(corners[i].x, corners[i].y, corners[i].z, 1.0);
			vertices.push(Vertex::new2(p.truncate(), uvs[i], WHITE));
		}

		indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
	}

	let mesh = Mesh { vertices, indices, texture: Some(atlas.clone()) };
	draw_mesh(&mesh);
}

fn draw_cube_outline(a: Vec3, b: Vec3)
{
	let a = a + b * 0.5;
	draw_cube_wires(a, b + vec3(EPS, EPS, EPS), WHITE);
}

fn draw_cube_with_outline(a: Vec3, b: Vec3, color: Color)
{
	let a = a + b * 0.5;
	draw_cube(a, b, None, color);
	draw_cube_wires(a, b + vec3(EPS, EPS, EPS), WHITE);
}

fn draw_inner_outline(pos: Vec3, h0: f32, h1: f32)
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

fn draw_door(pos: Vec3, door: f32, color: Color)
{
	let dw = door * (WIDTH - 2.0 * DOOR_X - GAP) / 2.0;

	draw_cube_with_outline(pos + vec3(DOOR_X, GAP / 2.0, WALL / 4.0),
		vec3(dw, HEIGHT - 2.0 * WALL - GAP, WALL / 2.0), color);

	draw_cube_with_outline(pos + vec3(WIDTH - DOOR_X - dw, GAP / 2.0, WALL / 4.0),
		vec3(dw, HEIGHT - 2.0 * WALL - GAP, WALL / 2.0), color);
}

fn draw_elevator(pos: Vec3, door: f32)
{
	// Bottom Base
	draw_my_cube(pos + vec3(0.0, -WALL, 0.0), vec3(WIDTH, WALL, DEPTH));

	// Top Plate
	draw_my_cube(pos + vec3(0.0, HEIGHT - 2.0 * WALL, 0.0), vec3(WIDTH, WALL, DEPTH));

	// Back Plate
	draw_my_cube(pos + vec3(0.0, 0.0, DEPTH - WALL),
		vec3(WIDTH, HEIGHT - 2.0 * WALL, WALL));

	// Front Segments
	draw_my_cube(pos + vec3(0.0, 0.0, 0.0),
		vec3(DOOR_X, HEIGHT - 2.0 * WALL, WALL));

	draw_my_cube(pos + vec3(WIDTH - DOOR_X, 0.0, 0.0),
		vec3(DOOR_X, HEIGHT - 2.0 * WALL, WALL));

	// Left and Right Side
	draw_my_cube(pos + vec3(0.0, 0.0, WALL),
		vec3(WALL, HEIGHT - 2.0 * WALL, DEPTH - 2.0 * WALL));

	draw_my_cube(pos + vec3(WIDTH - WALL, 0.0, WALL),
		vec3(WALL, HEIGHT - 2.0 * WALL, DEPTH - 2.0 * WALL));

	// Outline Outer
	draw_cube_outline(pos + vec3(0.0, -WALL, 0.0), vec3(WIDTH, HEIGHT, DEPTH));

	// Outline Inner
	draw_inner_outline(pos, EPS, HEIGHT - 2.0 * WALL - EPS);

	draw_door(pos, door, DOOR_COLOR);
}

const fn t(x: i32, y: i32, w: i32, h: i32) -> Rect
{
	const TW: f32 = 64.0;
	Rect::new(x as f32 / TW, y as f32 / TW, w as f32 / TW, h as f32 / TW)
}

fn draw_person(pos: Vec3, scale: f32, arm_thin: bool, texture: &Texture2D)
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

	let head_tex =
	[
		t(8, 8, 8, 8), t(24, 8, 8, 8),
		t(16, 8, 8, 8), t(0, 8, 8, 8),
		t(8, 0, 8, 8), t(16, 0, 8, 8)
	];

	let right_leg_tex =
	[
		t(4, 20, 4, 12), t(12, 20, 4, 12),
		t(0, 20, 4, 12), t(8, 20, 4, 12),
		t(4, 16, 4, 4),  t(8, 16, 4, 4)
	];

	let left_leg_tex =
	[
		t(20, 52, 4, 12), t(28, 52, 4, 12),
		t(16, 52, 4, 12), t(24, 52, 4, 12),
		t(20, 48, 4, 4),  t(24, 48, 4, 4)
	];

	let body_tex =
	[
		t(20, 20, 8, 12), t(32, 20, 8, 12),
		t(16, 20, 4, 12), t(28, 20, 4, 12),
		t(20, 16, 8, 4),  t(28, 16, 8, 4)
	];

	let left_arm_tex =
	[
		t(36, 52, 4, 12), t(44, 52, 4, 12),
		t(32, 52, 4, 12), t(40, 52, 4, 12),
		t(36, 48, 4, 4),  t(40, 48, 4, 4)
	];

	let right_arm_tex =
	[
		t(44, 20, 4, 12), t(52, 20, 4, 12),
		t(40, 20, 4, 12), t(48, 20, 4, 12),
		t(44, 16, 4, 4),  t(48, 16, 4, 4)
	];

	let left_arm_tex_slim =
	[
		t(36, 52, 3, 12), t(43, 52, 3, 12),
		t(32, 52, 4, 12), t(39, 52, 4, 12),
		t(36, 48, 3, 4),  t(39, 48, 3, 4)
	];

	let right_arm_tex_slim =
	[
		t(44, 20, 3, 12), t(51, 20, 3, 12),
		t(40, 20, 4, 12), t(47, 20, 4, 12),
		t(44, 16, 3, 4),  t(47, 16, 3, 4)
	];

	// Legs
	draw_tex_cube(pos + vec3(-leg_width, 0.0, -leg_width / 2.0),
		vec3(leg_width, leg_height, leg_width),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, left_leg_tex);

	draw_tex_cube(pos + vec3(0.0, 0.0, -leg_width / 2.0),
		vec3(leg_width, leg_height, leg_width),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, right_leg_tex);

	// Arms
	draw_tex_cube(pos + vec3(-leg_width - arm_wide, leg_height, -arm_thick / 2.0),
		vec3(arm_wide, arm_height, arm_thick),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, if arm_thin { right_arm_tex_slim } else { right_arm_tex });

	draw_tex_cube(pos + vec3(leg_width, leg_height, -arm_thick / 2.0),
		vec3(arm_wide, arm_height, arm_thick),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, if arm_thin { left_arm_tex_slim } else { left_arm_tex });

	// Body
	draw_tex_cube(pos + vec3(-body_width / 2.0, leg_height, -body_thick / 2.0),
		vec3(body_width, body_height, body_thick),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, body_tex);

	// Head
	draw_tex_cube(pos + vec3(-head_size / 2.0, leg_height + body_height, -head_size / 2.0),
		vec3(head_size, head_size, head_size),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, head_tex);
}

fn render_scene(cabins: &[Cabin], floors: &[Floor])
{
	let w = CABIN_STEP * (NUM_CABINS - 1) as f32 + WIDTH + 2.0 * WALL;

	for (i, floor) in floors.iter().enumerate().take(NUM_FLOORS)
	{
		let y = i as f32 * FLOOR_HEIGHT - WALL;

		draw_cube_with_outline(vec3(0.0, y, 0.0),
			vec3(w, WALL, FLOOR_D), PL_COLOR);

		draw_cube_with_outline(vec3(0.0, y + WALL + 2.0 * EPS, FLOOR_D - WALL),
			vec3(WALL + DOOR_X, HEIGHT, WALL), PL_COLOR);
		draw_cube_with_outline(vec3(w - WALL - DOOR_X, y + WALL + 2.0 * EPS, FLOOR_D - WALL),
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
	}

	for (i, cabin) in cabins.iter().enumerate()
	{
		draw_elevator(vec3(i as f32 * CABIN_STEP + WALL, cabin.y, FLOOR_D + GAP), cabin.door);
	}
}

fn update(cabins: &mut [Cabin], floors: &mut [Floor])
{
	let t = get_frame_time();

	for (i, cabin) in cabins.iter_mut().enumerate()
	{
		match cabin.state
		{
			CabinState::Stopped => {},
			CabinState::DoorClosing =>
			{
				cabin.door += DOOR_SPEED * t;
				if cabin.door >= 1.0
				{
					cabin.door = 1.0;
					cabin.state = CabinState::Moving;
				}

				floors[cabin.from].doors[i] = cabin.door;
			},
			CabinState::DoorOpening =>
			{
				cabin.door -= DOOR_SPEED * t;
				if cabin.door <= 0.0
				{
					cabin.door = 0.0;
					cabin.state = CabinState::Stopped;
					cabin.from = cabin.target;
				}

				floors[cabin.target].doors[i] = cabin.door;
			},
			CabinState::Moving =>
			{
				let target_y = cabin.target as f32 * FLOOR_HEIGHT;
				if cabin.y < target_y
				{
					cabin.y += CABIN_SPEED * t;
					if cabin.y >= target_y
					{
						cabin.y = target_y;
						cabin.state = CabinState::DoorOpening;
					}
				}
				else if cabin.y > target_y
				{
					cabin.y -= CABIN_SPEED * t;
					if cabin.y <= target_y
					{
						cabin.y = target_y;
						cabin.state = CabinState::DoorOpening;
					}
				}
			}
		}
	}
}

fn manual_control(sel_cabin: &mut usize, cabins: &mut [Cabin])
{
	if is_key_pressed(KeyCode::Z) { *sel_cabin = 0; }
	if is_key_pressed(KeyCode::U) { *sel_cabin = 1; }
	if is_key_pressed(KeyCode::I) { *sel_cabin = 2; }

	let mut sel_target = None;
	if is_key_pressed(KeyCode::Key1) { sel_target = Some(0); }
	if is_key_pressed(KeyCode::Key2) { sel_target = Some(1); }
	if is_key_pressed(KeyCode::Key3) { sel_target = Some(2); }
	if is_key_pressed(KeyCode::Key4) { sel_target = Some(3); }

	if let Some(sel_target) = sel_target
	{
		let cabin = &mut cabins[*sel_cabin];
		if cabin.state == CabinState::Stopped && cabin.target != sel_target
		{
			cabin.target = sel_target;
			cabin.state = CabinState::DoorClosing;
		}
	}
}

struct Player
{
	slim: bool,
	texture: Texture2D
}

async fn load_players() -> Vec<Player>
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

	let mut players = Vec::with_capacity(12);
	for (filename, slim) in filenames
	{
		let texture = load_texture(filename).await.unwrap();
		texture.set_filter(FilterMode::Nearest);
		players.push(Player { slim, texture });
	}

	players
}

#[macroquad::main("Fahrstuhl")]
async fn main()
{
	let mut camera_params = CameraParams::new(vec3(0.0, 1.5, 5.0));

	let mut cabins: [Cabin; NUM_CABINS] = [Cabin::new(); NUM_CABINS];
	let mut floors: [Floor; NUM_FLOORS] = [Floor::new(); NUM_FLOORS];

	for i in 0..NUM_CABINS
	{
		floors[0].doors[i] = 0.0;
	}

	let mut sel_cabin = 0;
	let players = load_players().await;

	loop
	{
		clear_background(BLACK);
		draw_text("PSZ-02: Fahrstuhl", 10.0, 25.0, 30.0, WHITE);

		camera_params.update();

		manual_control(&mut sel_cabin, &mut cabins);
		update(&mut cabins, &mut floors);

		render_scene(&cabins, &floors);
		for (i, player) in players.iter().enumerate()
		{
			draw_person(vec3(i as f32, 0.0, 0.0), 0.03, player.slim, &player.texture);
		}

		set_default_camera();
		next_frame().await
	}
}
