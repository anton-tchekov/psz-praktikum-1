use std::thread;
use std::sync::mpsc;
use std::collections::HashSet;
use std::time::Duration;
use macroquad::prelude::*;
use crate::constants::*;
use crate::cube::*;
use crate::skin::*;
use crate::render::*;
use crate::event::*;

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerState
{
	Idle,
	GotoCabin,
	Entering,
	Riding,
	Leaving,
	GotoEnd,
	End
}

pub struct Player
{
	pub target_pos: Vec3,
	pub pos: Vec3,
	pub rot: f32,
	pub walking: bool,
	pub skin_id: usize,
	pub state: PlayerState,
	pub cabin_id: usize,
	pub goal_floor: usize
}

pub struct PlayerVisual
{
	pub pos: Vec3,
	pub rot: f32,
	pub skin_id: usize,
	pub goal_floor: usize
}

impl PlayerVisual
{
	pub fn draw(&self, skins: &[Skin])
	{
		let skin = &skins[self.skin_id];
		draw_player(self.pos, self.rot, skin.slim, &skin.texture);
		draw_digit(self.pos + vec3(0.0, 1.0, 0.0), WHITE, self.goal_floor + 1, 0.1);
	}
}

fn choose_target_floor(cur: usize) -> usize
{
	let mut goal;
	while {
		goal = rand::gen_range(0, NUM_FLOORS);
		goal == cur
	} {}

	goal
}

fn choose_skin(used_skins: &mut HashSet<usize>, max_players: usize) -> usize
{
	let mut skin_id;
	while {
		skin_id = rand::gen_range(0, max_players);
		used_skins.contains(&skin_id)
	} {}

	used_skins.insert(skin_id);
	skin_id
}

pub fn draw_players<'a>(players: impl IntoIterator<Item = &'a PlayerVisual>,
	skins: &[Skin])
{
	for player in players
	{
		player.draw(skins);
	}
}

pub fn spawn_players(used_skins: &mut HashSet<usize>,
	max_players: usize, tx: &mpsc::Sender<Event>)
{
	if used_skins.len() < max_players &&
		rand::gen_range(0.0, 1.0) < get_frame_time() * SPAWN_RATE
	{
		Player::new(used_skins, max_players, tx);
	}
}

impl Player
{
	pub fn new<'a>(players: &mut HashSet<usize>,
		max_players: usize,
		txr: &mpsc::Sender<Event>)
	{
		let floor = rand::gen_range(0, NUM_FLOORS);
		let y = floor as f32 * FLOOR_HEIGHT;
		let skin_id = choose_skin(players, max_players);

		let mut player = Player {
			pos: vec3(0.0, y, 0.0),
			target_pos: vec3(0.0, y, 0.0),
			rot: 0.0,
			walking: false,
			skin_id,
			state: PlayerState::Idle,
			cabin_id: 0,
			goal_floor: choose_target_floor(floor)
		};

		let tx = txr.clone();
		thread::spawn(move ||
		{
			let thread_id = thread::current().id();
			let _ = tx.send(Event::PlayerRequest(HallRequest {
				from: floor,
				up: player.goal_floor > floor
			}));
			loop
			{
				match player.state
				{
					PlayerState::Idle =>
					{
						if player.pos.distance(player.target_pos) < 0.3
						{
							player.target_pos = vec3(
								rand::gen_range(FLOOR_BORDER, FLOOR_W - FLOOR_BORDER),
								player.pos.y,
								rand::gen_range(FLOOR_BORDER, FLOOR_D - FLOOR_BORDER));
						}
					},
					PlayerState::GotoCabin =>
					{

					},
					PlayerState::Entering =>
					{
						let entered = false;
						if entered
						{
							player.state = PlayerState::Riding;
							let _ = tx.send(Event::ButtonPressed(ElevatorRequest {
								cabin: player.cabin_id,
								to: player.goal_floor
							}));
						}
					},
					PlayerState::Riding =>
					{
						// Do Nothing, until arrive
					},
					PlayerState::Leaving =>
					{

					},
					PlayerState::GotoEnd =>
					{

					},
					PlayerState::End =>
					{
						let _ = tx.send(Event::PlayerDespawn(thread_id));
						return;
					}
				}

				if player.pos.distance(player.target_pos) < 0.3
				{
					player.walking = false;
				}
				else
				{
					player.walking = true;

					let dir = (player.target_pos - player.pos).normalize();
					let upd = dir * 0.02 * PLAYER_SPEED;

					player.rot = f32::atan2(dir.x, dir.z);
					player.pos += upd;
				}

				let _ = tx.send(Event::PlayerUpdate(thread_id,
					PlayerVisual
					{
						pos: player.pos,
						rot: player.rot,
						skin_id: player.skin_id,
						goal_floor: player.goal_floor
					}));

				thread::sleep(Duration::from_millis(20));
			}
		});
	}
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

const HEAD2_TEX: [Rect; 6] =
[
	t(32 + 8, 8, 8, 8),  t(32 + 24, 8, 8, 8),
	t(32 + 16, 8, 8, 8), t(32 + 0, 8, 8, 8),
	t(32 + 8, 0, 8, 8),  t(32 + 16, 0, 8, 8)
];

const RIGHT_LEG_TEX: [Rect; 6] =
[
	t(4, 20, 4, 12), t(12, 20, 4, 12),
	t(0, 20, 4, 12), t(8, 20, 4, 12),
	t(4, 16, 4, 4),  t(8, 16, 4, 4)
];

const RIGHT_LEG2_TEX: [Rect; 6] =
[
	t(4, 16 + 20, 4, 12), t(12, 16 + 20, 4, 12),
	t(0, 16 + 20, 4, 12), t( 8, 16 + 20, 4, 12),
	t(4, 16 + 16, 4, 4),  t( 8, 16 + 16, 4, 4)
];

const LEFT_LEG_TEX: [Rect; 6] =
[
	t(20, 52, 4, 12), t(28, 52, 4, 12),
	t(16, 52, 4, 12), t(24, 52, 4, 12),
	t(20, 48, 4, 4),  t(24, 48, 4, 4)
];

const LEFT_LEG2_TEX: [Rect; 6] =
[
	t(20 - 16, 52, 4, 12), t(28 - 16, 52, 4, 12),
	t(16 - 16, 52, 4, 12), t(24 - 16, 52, 4, 12),
	t(20 - 16, 48, 4, 4),  t(24 - 16, 48, 4, 4)
];

const BODY_TEX: [Rect; 6] =
[
	t(20, 20, 8, 12), t(32, 20, 8, 12),
	t(16, 20, 4, 12), t(28, 20, 4, 12),
	t(20, 16, 8, 4),  t(28, 16, 8, 4)
];

const BODY2_TEX: [Rect; 6] =
[
	t(20, 16 + 20, 8, 12), t(32, 16 + 20, 8, 12),
	t(16, 16 + 20, 4, 12), t(28, 16 + 20, 4, 12),
	t(20, 16 + 16, 8, 4),  t(28, 16 + 16, 8, 4)
];

const LEFT_ARM_TEX: [Rect; 6] =
[
	t(36, 52, 4, 12), t(44, 52, 4, 12),
	t(32, 52, 4, 12), t(40, 52, 4, 12),
	t(36, 48, 4, 4),  t(40, 48, 4, 4)
];

const LEFT_ARM2_TEX: [Rect; 6] =
[
	t(16 + 36, 52, 4, 12), t(16 + 44, 52, 4, 12),
	t(16 + 32, 52, 4, 12), t(16 + 40, 52, 4, 12),
	t(16 + 36, 48, 4, 4),  t(16 + 40, 48, 4, 4)
];

const RIGHT_ARM_TEX: [Rect; 6] =
[
	t(44, 20, 4, 12), t(52, 20, 4, 12),
	t(40, 20, 4, 12), t(48, 20, 4, 12),
	t(44, 16, 4, 4),  t(48, 16, 4, 4)
];

const RIGHT_ARM2_TEX: [Rect; 6] =
[
	t(44, 16 + 20, 4, 12), t(52, 16 + 20, 4, 12),
	t(40, 16 + 20, 4, 12), t(48, 16 + 20, 4, 12),
	t(44, 16 + 16, 4, 4),  t(48, 16 + 16, 4, 4)
];

const LEFT_ARM_TEX_SLIM: [Rect; 6] =
[
	t(36, 52, 3, 12), t(43, 52, 3, 12),
	t(32, 52, 4, 12), t(39, 52, 4, 12),
	t(36, 48, 3, 4),  t(39, 48, 3, 4)
];

const LEFT_ARM2_TEX_SLIM: [Rect; 6] =
[
	t(36 + 14, 52, 3, 12), t(43 + 14, 52, 3, 12),
	t(32 + 14, 52, 4, 12), t(39 + 14, 52, 4, 12),
	t(36 + 14, 48, 3, 4),  t(39 + 14, 48, 3, 4)
];

const RIGHT_ARM_TEX_SLIM: [Rect; 6] =
[
	t(44, 20, 3, 12), t(51, 20, 3, 12),
	t(40, 20, 4, 12), t(47, 20, 4, 12),
	t(44, 16, 3, 4),  t(47, 16, 3, 4)
];

const RIGHT_ARM2_TEX_SLIM: [Rect; 6] =
[
	t(44, 20 + 16, 3, 12), t(51, 20 + 16, 3, 12),
	t(40, 20 + 16, 4, 12), t(47, 20 + 16, 4, 12),
	t(44, 16 + 16, 3, 4),  t(47, 16 + 16, 3, 4)
];

struct Layer
{
	head: [Rect; 6],
	body: [Rect; 6],
	left_arm: [Rect; 6],
	right_arm: [Rect; 6],
	left_leg: [Rect; 6],
	right_leg: [Rect; 6],
	left_arm_slim: [Rect; 6],
	right_arm_slim: [Rect; 6]
}

const LAYER_INNER: Layer = Layer {
	head: HEAD_TEX,
	body: BODY_TEX,
	left_arm: LEFT_ARM_TEX,
	right_arm: RIGHT_ARM_TEX,
	left_leg: LEFT_LEG_TEX,
	right_leg: RIGHT_LEG_TEX,
	left_arm_slim: LEFT_ARM_TEX_SLIM,
	right_arm_slim: RIGHT_ARM_TEX_SLIM
};

const LAYER_OUTER: Layer = Layer {
	head: HEAD2_TEX,
	body: BODY2_TEX,
	left_arm: LEFT_ARM2_TEX,
	right_arm: RIGHT_ARM2_TEX,
	left_leg: LEFT_LEG2_TEX,
	right_leg: RIGHT_LEG2_TEX,
	left_arm_slim: LEFT_ARM2_TEX_SLIM,
	right_arm_slim: RIGHT_ARM2_TEX_SLIM
};

fn draw_layer(pos: Vec3, rot: f32, size: f32, mut scale: f32, arm_thin: bool,
	texture: &Texture2D, layer: &Layer)
{
	let head_size = 8.0 * size;
	let arm_wide = if arm_thin { 3.0 } else { 4.0 } * size;
	let arm_thick = 4.0 * size;
	let arm_height = 12.0 * size;
	let leg_width = 4.0 * size;
	let leg_height = 12.0 * size;
	let body_width = 8.0 * size;
	let body_thick = 4.0 * size;
	let body_height = 12.0 * size;

	let transform = Mat4::from_translation(pos) *
		Mat4::from_rotation_y(rot) * Mat4::from_translation(-pos);

	let angle = (get_time() * 5.0).sin() as f32 * 30.0_f32.to_radians();

	// Legs
	draw_cube_textured(transform, pos + vec3(-leg_width, 0.0, -leg_width / 2.0),
		vec3(leg_width, leg_height, leg_width),
		pos + vec3(0.0, leg_height, 0.0), vec3(angle, 0.0, 0.0),
		texture, layer.left_leg, scale);

	scale += 0.0001;

	draw_cube_textured(transform, pos + vec3(0.0, 0.0, -leg_width / 2.0),
		vec3(leg_width, leg_height, leg_width),
		pos + vec3(0.0, leg_height, 0.0), vec3(-angle, 0.0, 0.0),
		texture, layer.right_leg, scale);

	scale += 0.0001;

	// Arms
	draw_cube_textured(transform, pos + vec3(-leg_width - arm_wide, leg_height, -arm_thick / 2.0),
		vec3(arm_wide, arm_height, arm_thick),
		pos + vec3(0.0, leg_height + body_height - arm_thick / 2.0, 0.0), vec3(-angle, 0.0, 0.0),
		texture, if arm_thin { layer.right_arm_slim } else { layer.right_arm }, scale);

	scale += 0.0001;

	draw_cube_textured(transform, pos + vec3(leg_width, leg_height, -arm_thick / 2.0),
		vec3(arm_wide, arm_height, arm_thick),
		pos + vec3(0.0, leg_height + body_height - arm_thick / 2.0, 0.0), vec3(angle, 0.0, 0.0),
		texture, if arm_thin { layer.left_arm_slim } else { layer.left_arm }, scale);

	scale += 0.0001;

	// Body
	draw_cube_textured(transform, pos + vec3(-body_width / 2.0, leg_height, -body_thick / 2.0),
		vec3(body_width, body_height, body_thick),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, layer.body, scale);

	scale += 0.0001;

	// Head
	draw_cube_textured(transform, pos + vec3(-head_size / 2.0, leg_height + body_height, -head_size / 2.0),
		vec3(head_size, head_size, head_size),
		vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
		texture, layer.head, scale);
}

pub fn draw_player(pos: Vec3, rot: f32, arm_thin: bool, texture: &Texture2D)
{
	let size = 0.03;
	draw_layer(pos, rot, size, 0.0, arm_thin, texture, &LAYER_INNER);
	draw_layer(pos, rot, size, 0.01, arm_thin, texture, &LAYER_OUTER);
}
