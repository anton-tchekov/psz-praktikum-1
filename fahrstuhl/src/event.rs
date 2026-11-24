use std::thread;
use crate::player::PlayerVisual;

pub struct HallRequest
{
	pub from: usize,
	pub up: bool
}

pub struct ElevatorRequest
{
	pub cabin: usize,
	pub to: usize
}

pub enum Event
{
	ButtonPressed(ElevatorRequest),
	PlayerRequest(HallRequest),
	PlayerUpdate(thread::ThreadId, PlayerVisual),
	PlayerDespawn(thread::ThreadId)
}
