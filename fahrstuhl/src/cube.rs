use macroquad::prelude::*;
use crate::constants::*;

pub fn draw_cube_colored(a: Vec3, b: Vec3, color: Color)
{
	let a = a + b * 0.5;
	draw_cube(a, b, None, color);
}

pub fn draw_cube_textured(pos: Vec3, size: Vec3, rot_point: Vec3, rotation: Vec3,
	atlas: &Texture2D, rect: [Rect; 6], scale: f32)
{
	let rotation_matrix = Mat4::from_rotation_z(rotation.z) *
		Mat4::from_rotation_y(rotation.y) * Mat4::from_rotation_x(rotation.x);

	let transform = Mat4::from_translation(rot_point) *
		rotation_matrix * Mat4::from_translation(-rot_point);

	let (x0, y0, z0) = (pos.x - scale, pos.y - scale, pos.z - scale);
	let (x1, y1, z1) = (pos.x + size.x + scale, pos.y + size.y + scale, pos.z + size.z + scale);

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

pub fn draw_cube_outline(a: Vec3, b: Vec3)
{
	let a = a + b * 0.5;
	draw_cube_wires(a, b + vec3(EPS, EPS, EPS), WHITE);
}

pub fn draw_cube_with_outline(a: Vec3, b: Vec3, color: Color)
{
	let a = a + b * 0.5;
	draw_cube(a, b, None, color);
	draw_cube_wires(a, b + vec3(EPS, EPS, EPS), WHITE);
}
