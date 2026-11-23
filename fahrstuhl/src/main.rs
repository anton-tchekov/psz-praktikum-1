mod cabin;
mod floor;
mod cube;
mod player;
mod skin;
mod render;
mod camera_params;
mod constants;

use macroquad::prelude::*;

use crate::camera_params::*;
use crate::constants::*;
use crate::floor::*;
use crate::cabin::*;
use crate::skin::*;
use crate::render::*;
use crate::player::*;

#[derive(Copy, Clone, PartialEq)]
enum PassengerState
{
	Idle,
	Entering,
	Choosing,
	Riding,
	Leaving
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

#[macroquad::main("Fahrstuhl")]
async fn main()
{
	let mut camera_params = CameraParams::new(vec3(-4.85, 7.08, -11.9), 2.65, -0.13);

	let mut cabins: [Cabin; NUM_CABINS] = [Cabin::new(); NUM_CABINS];
	let mut floors: [Floor; NUM_FLOORS] = [Floor::new(); NUM_FLOORS];

	for i in 0..NUM_CABINS
	{
		floors[0].doors[i] = 0.0;
	}

	let mut sel_cabin = 0;
	let skins = load_skins().await;

	loop
	{
		clear_background(BLACK);
		draw_text("PSZ-02: Fahrstuhl", 10.0, 25.0, 30.0, WHITE);

		draw_text(format!("X = {:.02} | Y = {:.02} | Z = {:.02}",
			camera_params.pos.x, camera_params.pos.y, camera_params.pos.z),
			10.0, 75.0, 24.0, WHITE);

		draw_text(format!("Yaw = {:.02} | Pitch = {:.02}",
			camera_params.yaw.to_degrees(), camera_params.pitch.to_degrees()),
			10.0, 100.0, 24.0, WHITE);

		manual_control(&mut sel_cabin, &mut cabins);
		update(&mut cabins, &mut floors);

		camera_params.update();
		draw_scene(&cabins, &floors);
		for (i, skin) in skins.iter().enumerate()
		{
			draw_player(vec3(i as f32, 0.0, 0.0), i as f32 * 45.0_f32.to_radians(), skin.slim, &skin.texture);
			draw_digit(vec3(i as f32, 1.0, 0.0), WHITE, i % 10, 0.1);
		}

		set_default_camera();
		next_frame().await
	}
}
