mod cabin;
mod floor;
mod cube;
mod player;
mod skin;
mod render;
mod camera_params;
mod constants;
mod event;

use std::sync::mpsc;
use std::collections::{HashSet, HashMap};

use macroquad::prelude::*;
use crate::event::*;
use crate::camera_params::*;
use crate::constants::*;
use crate::floor::*;
use crate::cabin::*;
use crate::skin::*;
use crate::render::*;
use crate::player::*;

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
	let (tx, rx) = mpsc::channel();
	let mut sel_cabin = 0;
	let skins = load_skins().await;
	//let mut players = Vec::with_capacity(skins.len());
	let mut camera_params = CameraParams::new(vec3(-4.85, 7.08, -11.9), 2.65, -0.13);
	let mut cabins: [Cabin; NUM_CABINS] = [Cabin::new(0), Cabin::new(1), Cabin::new(2)];
	let mut floors: [Floor; NUM_FLOORS] = [Floor::new(); NUM_FLOORS];
	for i in 0..NUM_CABINS
	{
		floors[0].doors[i] = 0.0;
	}

	let mut used_skins = HashSet::new();
	let mut player_visuals = HashMap::new();

	rand::srand(macroquad::miniquad::date::now() as _);
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

		print_cabin_queue(&cabins);

		manual_control(&mut sel_cabin, &mut cabins);

		while let Ok(event) = rx.try_recv()
		{
			match event
			{
				Event::PlayerRequest(hall_req) =>
				{
					let floor = &mut floors[hall_req.from];
					if hall_req.up
					{
						if !floor.indicator_up_on
						{
							floor.indicator_up_on = true;
							cabins_add_request(hall_req.from, hall_req.up, &mut cabins);
						}
					}
					else
					{
						if !floor.indicator_down_on
						{
							floor.indicator_down_on = true;
							cabins_add_request(hall_req.from, hall_req.up, &mut cabins);
						}
					}
				},
				Event::ButtonPressed(elevator_req) =>
				{
					cabins[elevator_req.cabin].add_queue(elevator_req.to);
				},
				Event::PlayerUpdate(thread_id, visual) =>
				{
					player_visuals.insert(thread_id, visual);
				},
				Event::PlayerDespawn(thread_id) =>
				{
					player_visuals.remove(&thread_id);
				}
			}
		}

		spawn_players(&mut used_skins, skins.len(), &tx);
		update_cabins(&mut cabins, &mut floors);

		camera_params.update();
		draw_scene(&cabins, &floors);
		draw_players(player_visuals.values(), &skins);
		set_default_camera();
		next_frame().await
	}
}
