#[derive(Copy, Clone, PartialEq)]
pub enum CabinState
{
	DoorClosing,
	Moving,
	DoorOpening,
	Stopped
}

#[derive(Copy, Clone)]
pub struct Cabin
{
	pub y: f32,
	pub door: f32,
	pub from: usize,
	pub target: usize,
	pub state: CabinState
}

impl Cabin
{
	pub fn new() -> Self
	{
		Self { y: 0.0, door: 0.0, from: 0, target: 0, state: CabinState::Stopped }
	}
}
