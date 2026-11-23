use crate::constants::*;

#[derive(Copy, Clone)]
pub struct Floor
{
	pub doors: [f32; NUM_CABINS],
	pub indicator_up_on: bool,
	pub indicator_down_on: bool
}

impl Floor
{
	pub fn new() -> Self
	{
		Self
		{
			doors: [1.0; NUM_CABINS],
			indicator_up_on: true,
			indicator_down_on: true
		}
	}
}
