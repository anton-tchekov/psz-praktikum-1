use std::collections::VecDeque;
use macroquad::prelude::*;
use crate::floor::*;
use crate::constants::*;

#[derive(Copy, Clone, PartialEq)]
pub enum CabinState
{
	DoorClosing,
	Moving,
	DoorOpening,
	Stopped
}

//#[derive(Copy, Clone)]
pub struct Cabin
{
	pub id: usize,
	pub y: f32,
	pub door: f32,
	pub from: usize,
	pub target: usize,
	pub state: CabinState,
	pub queue: VecDeque<usize>
}

impl Cabin
{
	pub fn new(id: usize) -> Self
	{
		Self
		{
			id,
			y: 0.0,
			door: 0.0,
			from: 0,
			target: 0,
			state: CabinState::Stopped,
			queue: VecDeque::new()
		}
	}

	pub fn update(&mut self, floors: &mut [Floor])
	{
		let t = get_frame_time();
		match self.state
		{
			CabinState::Stopped => {},
			CabinState::DoorClosing =>
			{
				self.door += DOOR_SPEED * t;
				if self.door >= 1.0
				{
					self.door = 1.0;
					self.state = CabinState::Moving;
				}

				floors[self.from].doors[self.id] = self.door;
			},
			CabinState::DoorOpening =>
			{
				self.door -= DOOR_SPEED * t;
				if self.door <= 0.0
				{
					self.door = 0.0;
					self.state = CabinState::Stopped;
					self.from = self.target;
				}

				floors[self.target].doors[self.id] = self.door;
			},
			CabinState::Moving =>
			{
				let target_y = self.target as f32 * FLOOR_HEIGHT;
				if self.y < target_y
				{
					self.y += CABIN_SPEED * t;
					if self.y >= target_y
					{
						self.y = target_y;
						self.state = CabinState::DoorOpening;
					}
				}
				else if self.y > target_y
				{
					self.y -= CABIN_SPEED * t;
					if self.y <= target_y
					{
						self.y = target_y;
						self.state = CabinState::DoorOpening;
					}
				}
			}
		}
	}

	pub fn add_queue(&mut self, floor: usize)
	{
		if self.queue.back() != Some(&floor)
		{
			self.queue.push_back(floor);
		}
	}
}

pub fn update_cabins(cabins: &mut [Cabin], floors: &mut [Floor])
{
	for cabin in cabins.iter_mut()
	{
		cabin.update(floors);
	}
}

pub fn cabins_add_request(floor: usize, dir: bool, cabins: &mut [Cabin])
{
	let mut closest = 0;
	let mut min_dist = 1000.0;
	for (i, cabin) in cabins.iter().enumerate()
	{
		let elev_dir = cabin.target > cabin.from;
		let dir_penalty = if dir != elev_dir { 1.0 } else { 0.0 };
		let dist = ((cabin.y / FLOOR_HEIGHT) - (floor as f32)).abs() + dir_penalty;
		if dist < min_dist
		{
			min_dist = dist;
			closest = i;
		}
	}

	cabins[closest].add_queue(floor);
}

pub fn print_cabin_queue(cabins: &[Cabin])
{
	let mut y = 130.0;
	for (i, cabin) in cabins.iter().enumerate()
	{
		draw_text(format!("Cabin {}: {:?}", i, cabin.queue),
			10.0, y, 24.0, WHITE);

		y += 30.0;
	}
}
