use macroquad::prelude::*;

const MOVE_SPEED: f32 = 3.0;
const MOUSE_SENSITIVITY: f32 = 0.001;

pub struct CameraParams
{
	pub pos: Vec3,
	pub yaw: f32,
	pub pitch: f32,
	last_mouse: Vec2
}

impl CameraParams
{
	pub fn new(pos: Vec3, yaw: f32, pitch: f32) -> Self
	{
		Self
		{
			pos,
			yaw,
			pitch,
			last_mouse: mouse_position().into()
		}
	}

	pub fn update(&mut self)
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

		let mut movement = vec3(0.0, 0.0, 0.0);
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
			-self.yaw.cos() * self.pitch.cos());

		set_camera(&Camera3D
			{
				position: self.pos,
				target: self.pos + look_dir,
				up: vec3(0.0, 1.0, 0.0),
				..Default::default()
			});
	}
}
